import 'dart:typed_data';
import 'package:flutter/material.dart';
import '../models/visualizer_theme.dart';

class WaveformPainter extends CustomPainter {
  final Float32List spectrum;
  final VisualizerTheme theme;

  WaveformPainter({
    required this.spectrum,
    required this.theme,
  });

  @override
  void paint(Canvas canvas, Size size) {
    if (spectrum.isEmpty) return;

    final double centerY = size.height * 0.55;
    final int count = spectrum.length;
    final double stepX = size.width / (count - 1);

    final Path topPath = Path();
    final Path bottomPath = Path();

    topPath.moveTo(0, centerY);
    bottomPath.moveTo(0, centerY);

    for (int i = 0; i < count; i++) {
      final double x = i * stepX;
      final double magnitude = spectrum[i].clamp(0.0, 1.0);
      final double waveHeight = magnitude * (size.height * 0.4);

      final double yTop = centerY - waveHeight;
      final double yBottom = centerY + waveHeight;

      if (i == 0) {
        topPath.lineTo(x, yTop);
        bottomPath.lineTo(x, yBottom);
      } else {
        final double prevX = (i - 1) * stepX;
        final double prevMag = spectrum[i - 1].clamp(0.0, 1.0);
        final double prevTopY = centerY - prevMag * (size.height * 0.4);
        final double prevBottomY = centerY + prevMag * (size.height * 0.4);

        final double cX = (prevX + x) / 2;
        topPath.cubicTo(cX, prevTopY, cX, yTop, x, yTop);
        bottomPath.cubicTo(cX, prevBottomY, cX, yBottom, x, yBottom);
      }
    }

    // Mirror fill path
    final Path fillPath = Path.from(topPath);
    fillPath.lineTo(size.width, centerY);
    fillPath.lineTo(size.width, centerY);
    // trace bottom backwards
    for (int i = count - 1; i >= 0; i--) {
      final double x = i * stepX;
      final double magnitude = spectrum[i].clamp(0.0, 1.0);
      final double yBottom = centerY + magnitude * (size.height * 0.4);
      fillPath.lineTo(x, yBottom);
    }
    fillPath.close();

    final Paint fillPaint = Paint()
      ..shader = LinearGradient(
        begin: Alignment.topCenter,
        end: Alignment.bottomCenter,
        colors: [
          theme.secondary.withValues(alpha: 0.35),
          theme.primary.withValues(alpha: 0.15),
          theme.secondary.withValues(alpha: 0.35),
        ],
      ).createShader(Rect.fromLTWH(0, 0, size.width, size.height));
    canvas.drawPath(fillPath, fillPaint);

    final Paint strokePaint = Paint()
      ..shader = theme.barGradient.createShader(
        Rect.fromLTWH(0, 0, size.width, size.height),
      )
      ..style = PaintingStyle.stroke
      ..strokeWidth = 3.0
      ..strokeCap = StrokeCap.round;

    canvas.drawPath(topPath, strokePaint);
    canvas.drawPath(bottomPath, strokePaint);
  }

  @override
  bool shouldRepaint(covariant WaveformPainter oldDelegate) => true;
}
