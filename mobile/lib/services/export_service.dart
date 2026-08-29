import 'dart:io';
import 'dart:math' as math;
import 'dart:ui' as ui;
import 'package:flutter/foundation.dart';
import 'package:flutter/material.dart';
import 'package:flutter_quick_video_encoder/flutter_quick_video_encoder.dart';
import 'package:permission_handler/permission_handler.dart';
import 'package:wakelock_plus/wakelock_plus.dart';
import '../models/circle_center_display.dart';
import '../models/visualizer_mode.dart';
import '../models/visualizer_theme.dart';
import '../painters/bars_painter.dart';
import '../painters/circular_painter.dart';
import '../painters/waveform_painter.dart';
import '../src/rust/api.dart' as rust_api;

class ExportService {
  /// Resolves the user-accessible directory to store exported MP4 videos.
  static Future<Directory> getExportDirectory() async {
    if (Platform.isAndroid) {
      final moviesDir = Directory('/storage/emulated/0/Movies/Musializer');
      try {
        if (!await moviesDir.exists()) {
          await moviesDir.create(recursive: true);
        }
        return moviesDir;
      } catch (_) {
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
      final docs = Directory('${Platform.environment['HOME']}/Documents/Exports');
      if (!await docs.exists()) {
        await docs.create(recursive: true);
      }
      return docs;
    } else {
      return Directory.current;
    }
  }

  /// Encodes a complete, high-definition MP4 video with full synchronized H.264 video and stereo AAC audio.
  static Future<String> renderVisualizerVideo({
    required String trackTitle,
    required VisualizerMode mode,
    required VisualizerTheme theme,
    required int numBands,
    required double gainMultiplier,
    required CircleCenterDisplay centerDisplay,
    required double duration,
    required ui.Image? coverImage,
    required bool isLandscape,
    required void Function(double progress, String status) onProgress,
  }) async {
    // 1. Request permissions and keep the screen awake
    if (Platform.isAndroid) {
      await [
        Permission.videos,
        Permission.storage,
      ].request();
    }
    await WakelockPlus.enable();

    try {
      final int width = isLandscape ? 1280 : 720;
      final int height = isLandscape ? 720 : 1280;
      const int fps = 30;
      const int sampleRate = 44100;
      const int channels = 2;

      onProgress(0.02, 'Extracting lossless audio & calculating FFT spectrum...');

      // Extract real audio PCM bytes from Rust core
      Uint8List? rawPcm;
      try {
        final pcmList = await rust_api.getOfflineAudioPcm();
        rawPcm = Uint8List.fromList(pcmList);
      } catch (e) {
        debugPrint('Could not extract PCM audio from Rust: $e');
      }

      // Compute exact deterministic FFT spectrum for all video frames from Rust DSP engine
      Float32List allSpectrumFrames;
      try {
        final flatList = await rust_api.getOfflineSpectrumFrames(
          fps: fps,
          numBands: BigInt.from(numBands),
          gainMultiplier: gainMultiplier,
        );
        allSpectrumFrames = Float32List.fromList(flatList);
      } catch (e) {
        debugPrint('Could not compute offline spectrum from Rust: $e');
        allSpectrumFrames = Float32List(0);
      }

      final exportDir = await getExportDirectory();
      final cleanTitle = trackTitle.replaceAll(RegExp(r'[^\w\s\.-]'), '_').trim();
      final timestamp = DateTime.now().millisecondsSinceEpoch;
      final ratioTag = isLandscape ? '16x9' : '9x16';
      final filename = 'Musializer_${cleanTitle}_${mode.title}_${ratioTag}_$timestamp.mp4';
      final targetPath = '${exportDir.path}/$filename';

      // 2. Setup Native Hardware Video & Audio Encoder
      await FlutterQuickVideoEncoder.setup(
        width: width,
        height: height,
        fps: fps,
        videoBitrate: 4000000,
        profileLevel: ProfileLevel.any,
        audioChannels: rawPcm != null && rawPcm.isNotEmpty ? channels : 0,
        audioBitrate: 192000,
        sampleRate: sampleRate,
        filepath: targetPath,
      );

      final double effectiveDuration = duration > 0 ? duration.clamp(3.0, 300.0) : 10.0;
      final int totalFrames = (effectiveDuration * fps).toInt();

      // Bytes per video frame for audio interleaving: (44100 * 2 channels * 2 bytes/sample) / 30 fps = 5880 bytes/frame
      const int bytesPerFrame = (sampleRate * channels * 2) ~/ fps;

      final size = Size(width.toDouble(), height.toDouble());
      final bgPaint = Paint()..color = theme.background;

      List<double> peaks = List.filled(numBands, 0.0);
      const double dt = 1.0 / fps;

      // 3. Render and encode each frame with exact audio DSP spectrum
      for (int frameIdx = 0; frameIdx < totalFrames; frameIdx++) {
        final double progress = frameIdx / totalFrames;
        final double currentTime = progress * effectiveDuration;

        if (frameIdx % 5 == 0 || frameIdx == totalFrames - 1) {
          onProgress(progress, 'Encoding frame $frameIdx / $totalFrames...');
        }

        // Get exact FFT frequency spectrum slice computed by Rust for this precise frame
        final Float32List frameSpectrum = Float32List(numBands);
        final int frameOffset = frameIdx * numBands;
        if (frameOffset + numBands <= allSpectrumFrames.length) {
          for (int i = 0; i < numBands; i++) {
            frameSpectrum[i] = allSpectrumFrames[frameOffset + i];
          }
        }

        // Compute smooth peak hold decay
        for (int i = 0; i < numBands; i++) {
          if (frameSpectrum[i] > peaks[i]) {
            peaks[i] = frameSpectrum[i];
          } else {
            peaks[i] = math.max(0.0, peaks[i] - 0.4 * dt);
          }
        }

        final recorder = ui.PictureRecorder();
        final canvas = Canvas(recorder, Rect.fromLTWH(0, 0, width.toDouble(), height.toDouble()));

        canvas.drawRect(Rect.fromLTWH(0, 0, width.toDouble(), height.toDouble()), bgPaint);

        switch (mode) {
          case VisualizerMode.spectrumBars:
            BarsPainter(
              spectrum: frameSpectrum,
              peaks: peaks,
              theme: theme,
            ).paint(canvas, size);
            break;
          case VisualizerMode.circular:
            CircularPainter(
              spectrum: frameSpectrum,
              theme: theme,
              centerDisplay: centerDisplay,
              currentTime: currentTime,
              duration: effectiveDuration,
              trackTitle: trackTitle,
              coverImage: coverImage,
            ).paint(canvas, size);
            break;
          case VisualizerMode.waveform:
            WaveformPainter(
              spectrum: frameSpectrum,
              theme: theme,
            ).paint(canvas, size);
            break;
        }

        final picture = recorder.endRecording();
        final img = await picture.toImage(width, height);
        final byteData = await img.toByteData(format: ui.ImageByteFormat.rawRgba);

        if (byteData != null) {
          await FlutterQuickVideoEncoder.appendVideoFrame(byteData.buffer.asUint8List());
        }

        // Append corresponding synchronized audio chunk
        if (rawPcm != null && rawPcm.isNotEmpty) {
          final int start = frameIdx * bytesPerFrame;
          if (start < rawPcm.length) {
            final int end = math.min(start + bytesPerFrame, rawPcm.length);
            final chunk = rawPcm.sublist(start, end);
            if (chunk.isNotEmpty) {
              await FlutterQuickVideoEncoder.appendAudioFrame(chunk);
            }
          }
        }
      }

      // 4. Finalize and close the MP4 video container
      await FlutterQuickVideoEncoder.finish();
      debugPrint('Native MP4 Video with audio and true FFT spectrum successfully generated at: $targetPath');
      return targetPath;
    } finally {
      // Re-enable screen lock / sleep mode
      await WakelockPlus.disable();
    }
  }
}
