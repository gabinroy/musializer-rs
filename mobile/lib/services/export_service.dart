import 'dart:io';
import 'dart:ui' as ui;
import 'package:flutter/foundation.dart';
import 'package:flutter/material.dart';
import 'package:permission_handler/permission_handler.dart';
import '../models/circle_center_display.dart';
import '../models/visualizer_mode.dart';
import '../models/visualizer_theme.dart';
import '../painters/bars_painter.dart';
import '../painters/circular_painter.dart';
import '../painters/waveform_painter.dart';

class ExportService {
  /// Resolves user-accessible directory to store exported visualizer snapshots/videos.
  static Future<Directory> getExportDirectory() async {
    if (Platform.isAndroid) {
      final picturesDir = Directory('/storage/emulated/0/Pictures/Musializer');
      try {
        if (!await picturesDir.exists()) {
          await picturesDir.create(recursive: true);
        }
        return picturesDir;
      } catch (_) {
        final moviesDir = Directory('/storage/emulated/0/Movies/Musializer');
        try {
          if (!await moviesDir.exists()) {
            await moviesDir.create(recursive: true);
          }
          return moviesDir;
        } catch (_) {
          return Directory.systemTemp;
        }
      }
    } else if (Platform.isIOS) {
      final docs = Directory('${Platform.environment['HOME']}/Documents/Exports');
      if (!await docs.exists()) {
        await docs.create(recursive: true);
      }
      return docs;
    } else {
      return Directory.current;
    }
  }

  /// Renders a crisp, real raster visualizer graphic frame into device gallery.
  static Future<String> exportVisualizerImageFrame({
    required String trackTitle,
    required VisualizerMode mode,
    required VisualizerTheme theme,
    required Float32List spectrum,
    required List<double> peaks,
    required CircleCenterDisplay centerDisplay,
    required double currentTime,
    required double duration,
    required ui.Image? coverImage,
    required bool isLandscape,
  }) async {
    if (Platform.isAndroid) {
      await [
        Permission.photos,
        Permission.storage,
      ].request();
    }

    final double targetWidth = isLandscape ? 1920 : 1080;
    final double targetHeight = isLandscape ? 1080 : 1920;

    final recorder = ui.PictureRecorder();
    final canvas = Canvas(recorder, Rect.fromLTWH(0, 0, targetWidth, targetHeight));

    // 1. Draw solid dark background
    final bgPaint = Paint()..color = theme.background;
    canvas.drawRect(Rect.fromLTWH(0, 0, targetWidth, targetHeight), bgPaint);

    // 2. Draw actual visualizer mode
    final size = Size(targetWidth, targetHeight);
    switch (mode) {
      case VisualizerMode.spectrumBars:
        BarsPainter(
          spectrum: spectrum,
          peaks: peaks,
          theme: theme,
        ).paint(canvas, size);
        break;
      case VisualizerMode.circular:
        CircularPainter(
          spectrum: spectrum,
          theme: theme,
          centerDisplay: centerDisplay,
          currentTime: currentTime,
          duration: duration,
          trackTitle: trackTitle,
          coverImage: coverImage,
        ).paint(canvas, size);
        break;
      case VisualizerMode.waveform:
        WaveformPainter(
          spectrum: spectrum,
          theme: theme,
        ).paint(canvas, size);
        break;
    }

    final picture = recorder.endRecording();
    final img = await picture.toImage(targetWidth.toInt(), targetHeight.toInt());
    final pngBytes = await img.toByteData(format: ui.ImageByteFormat.png);

    if (pngBytes == null) {
      throw Exception('Failed to encode visualizer frame');
    }

    final exportDir = await getExportDirectory();
    final cleanTitle = trackTitle.replaceAll(RegExp(r'[^\w\s\.-]'), '_').trim();
    final timestamp = DateTime.now().millisecondsSinceEpoch;
    final ratioTag = isLandscape ? '16x9' : '9x16';
    final filename = 'Musializer_${cleanTitle}_${mode.title}_${ratioTag}_$timestamp.png';
    final targetFile = File('${exportDir.path}/$filename');

    await targetFile.writeAsBytes(pngBytes.buffer.asUint8List(), flush: true);
    debugPrint('Exported visualizer frame saved: ${targetFile.path}');
    return targetFile.path;
  }
}
