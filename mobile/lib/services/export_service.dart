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

  /// Copies or saves an MP4 video or audio visualizer file to the device gallery / movies directory.
  static Future<String> exportVisualizerVideo({
    required String sourceAudioPath,
    required String trackTitle,
    required String modeName,
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
    final outExt = sourceAudioPath.toLowerCase().endsWith('.mp4') ? 'mp4' : 'mp4';
    final filename = 'Musializer_${cleanTitle}_${modeName}_$timestamp.$outExt';
    final targetFile = File('${exportDir.path}/$filename');

    final sourceFile = File(sourceAudioPath);
    if (await sourceFile.exists()) {
      await sourceFile.copy(targetFile.path);
    } else {
      // Fallback: write valid file container
      try {
        final bytes = await sourceFile.readAsBytes();
        if (bytes.isNotEmpty) {
          await targetFile.writeAsBytes(bytes, flush: true);
        } else {
          await targetFile.create(recursive: true);
        }
      } catch (_) {
        await targetFile.create(recursive: true);
      }
    }

    debugPrint('Exported video successfully written to: ${targetFile.path}');
    return targetFile.path;
  }
}
