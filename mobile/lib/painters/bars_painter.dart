import 'dart:typed_data';
import 'package:flutter/material.dart';
import '../models/visualizer_theme.dart';

/// GPU canvas renderer for classic vertical frequency spectrum bars.
///
/// Draws rounded vertical bars with dynamic gradient shaders, bloom glow passes,
/// and floating white peak-decay caps.
class BarsPainter extends CustomPainter {
  /// Real-time smoothed frequency magnitudes in range `[0.0, 1.0]`.
  final Float32List spectrum;

  /// Floating peak-hold values for each bar.
  final List<double> peaks;

  /// Active color and gradient theme.
  final VisualizerTheme theme;

  BarsPainter({
    required this.spectrum,
    required this.peaks,
    required this.theme,
  });

  @override
  void paint(Canvas canvas, Size size) {
    if (spectrum.isEmpty) return;

    final int barCount = spectrum.length;
    final double totalSpacing = size.width * 0.15;
    final double barSpacing = totalSpacing / (barCount + 1);
    final double barWidth = (size.width - totalSpacing) / barCount;
    final double cornerRadius = barWidth * 0.4;

    final Paint barPaint = Paint()
      ..shader = theme.barGradient.createShader(
        Rect.fromLTWH(0, 0, size.width, size.height),
      );

    final Paint glowPaint = Paint()
      ..color = theme.glow
      ..maskFilter = const MaskFilter.blur(BlurStyle.normal, 12.0);

    final Paint peakPaint = Paint()
      ..color = Colors.white.withValues(alpha: 0.9)
      ..style = PaintingStyle.fill;

    for (int i = 0; i < barCount; i++) {
      final double magnitude = spectrum[i].clamp(0.0, 1.0);
      final double barHeight = (magnitude * (size.height - 40.0)).clamp(4.0, size.height);
      final double x = barSpacing + i * (barWidth + barSpacing);
      final double y = size.height - barHeight;

      final RRect rrect = RRect.fromRectAndRadius(
        Rect.fromLTWH(x, y, barWidth, barHeight),
        Radius.circular(cornerRadius),
      );

      // Glow layer
      if (magnitude > 0.3) {
        canvas.drawRRect(rrect, glowPaint);
      }

      // Bar layer
      canvas.drawRRect(rrect, barPaint);

      // Floating peak cap
      if (i < peaks.length) {
        final double peakMag = peaks[i].clamp(0.0, 1.0);
        final double peakY = size.height - (peakMag * (size.height - 40.0)).clamp(4.0, size.height);
        final double capHeight = 3.0;

        final RRect peakRRect = RRect.fromRectAndRadius(
          Rect.fromLTWH(x, peakY - capHeight - 2.0, barWidth, capHeight),
          Radius.circular(capHeight * 0.5),
        );
        canvas.drawRRect(peakRRect, peakPaint);
      }
    }
  }

  @override
  bool shouldRepaint(covariant BarsPainter oldDelegate) {
    return true; // Continuously animated by ticker
  }
}
