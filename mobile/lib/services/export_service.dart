import 'dart:io';
import 'package:flutter/foundation.dart';
import 'package:permission_handler/permission_handler.dart';

class ExportService {
  /// Resolves the user-accessible directory to store exported MP4 and audio visualizer files.
  static Future<Directory> getExportDirectory() async {
    if (Platform.isAndroid) {
      // 1. Try public Movies / Download folder on Android
      final moviesDir = Directory('/storage/emulated/0/Movies/Musializer');
      try {
        if (!await moviesDir.exists()) {
          await moviesDir.create(recursive: true);
        }
        return moviesDir;
      } catch (_) {
        // Fallback to Downloads
        final downloadsDir = Directory('/storage/emulated/0/Download/Musializer');
        try {
          if (!await downloadsDir.exists()) {
            await downloadsDir.create(recursive: true);
          }
          return downloadsDir;
        } catch (_) {
          return Directory.systemTemp;
        }
      }
    } else if (Platform.isIOS) {
      // iOS Documents directory is exposed via UIFileSharingEnabled in Info.plist
      final docs = Directory('${Platform.environment['HOME']}/Documents/Exports');
      if (!await docs.exists()) {
        await docs.create(recursive: true);
      }
      return docs;
    } else {
      return Directory.current;
    }
  }

  /// Exports an MP4 video or visualizer snapshot to the device gallery / movies directory.
  static Future<String> saveExportedVideo({
    required String trackTitle,
    required String modeName,
    required List<int> videoBytes,
  }) async {
    // Request storage / media permissions
    if (Platform.isAndroid) {
      await [
        Permission.videos,
        Permission.storage,
      ].request();
    }

    final exportDir = await getExportDirectory();
    final cleanTitle = trackTitle.replaceAll(RegExp(r'[^\w\s\.-]'), '_').trim();
    final timestamp = DateTime.now().millisecondsSinceEpoch;
    final filename = 'Musializer_${cleanTitle}_${modeName}_$timestamp.mp4';
    final file = File('${exportDir.path}/$filename');

    await file.writeAsBytes(videoBytes, flush: true);
    debugPrint('Exported video saved to: ${file.path}');
    return file.path;
  }
}
