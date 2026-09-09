import 'dart:typed_data';
import 'package:flutter/material.dart';
import '../models/visualizer_theme.dart';

/// GPU canvas renderer for smooth cubic bezier oscilloscopic audio waveforms.
///
/// Converts frequency spectrum energies into a mirrored, fluid fluid wave shape with
/// neon gradient fills and blurred stroke glow edges.
class WaveformPainter extends CustomPainter {
  /// Real-time smoothed frequency magnitudes in range `[0.0, 1.0]`.
  final Float32List spectrum;

  /// Active color and gradient theme.
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

    // Fill with gradient
    final Paint fillPaint = Paint()
      ..shader = theme.barGradient.createShader(Rect.fromLTWH(0, 0, size.width, size.height))
      ..style = PaintingStyle.fill;
    canvas.drawPath(fillPath, fillPaint);

    // Waveform Outline with Glow
    final Paint strokePaint = Paint()
      ..color = Colors.white.withValues(alpha: 0.9)
      ..style = PaintingStyle.stroke
      ..strokeWidth = 2.5
      ..strokeCap = StrokeCap.round;

    final Paint glowPaint = Paint()
      ..color = theme.glow
      ..style = PaintingStyle.stroke
      ..strokeWidth = 6.0
      ..maskFilter = const MaskFilter.blur(BlurStyle.normal, 8.0);

    canvas.drawPath(topPath, glowPaint);
    canvas.drawPath(topPath, strokePaint);
    canvas.drawPath(bottomPath, glowPaint);
    canvas.drawPath(bottomPath, strokePaint);
  }

  @override
  bool shouldRepaint(covariant WaveformPainter oldDelegate) => true;
}
