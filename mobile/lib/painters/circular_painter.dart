import 'dart:math' as math;
import 'dart:typed_data';
import 'package:flutter/material.dart';
import '../models/visualizer_theme.dart';

class CircularPainter extends CustomPainter {
  final Float32List spectrum;
  final VisualizerTheme theme;

  CircularPainter({
    required this.spectrum,
    required this.theme,
  });

  @override
  void paint(Canvas canvas, Size size) {
    final Offset center = Offset(size.width / 2, size.height / 2);
    final double maxRadius = math.min(size.width, size.height) * 0.45;
    
    // Bass reactive pulse (average of first 6 bins)
    double bassSum = 0.0;
    final int bassCount = math.min(spectrum.length, 6);
    for (int i = 0; i < bassCount; i++) {
      bassSum += spectrum[i];
    }
    final double bassEnergy = bassCount > 0 ? (bassSum / bassCount) : 0.0;
    final double baseRadius = maxRadius * 0.45 + (bassEnergy * 25.0);

    // Inner Glow Circle
    final Paint centerGlow = Paint()
      ..color = theme.glow.withValues(alpha: 0.2 + (bassEnergy * 0.3))
      ..maskFilter = const MaskFilter.blur(BlurStyle.normal, 24.0);
    canvas.drawCircle(center, baseRadius * 0.9, centerGlow);

    // Inner Core Circle
    final Paint corePaint = Paint()
      ..shader = RadialGradient(
        colors: [theme.primary.withValues(alpha: 0.8), theme.background],
      ).createShader(Rect.fromCircle(center: center, radius: baseRadius));
    canvas.drawCircle(center, baseRadius, corePaint);

    if (spectrum.isEmpty) return;

    final int count = spectrum.length;
    final double angleStep = (2 * math.pi) / count;

    final Paint rayPaint = Paint()
      ..style = PaintingStyle.stroke
      ..strokeCap = StrokeCap.round
      ..strokeWidth = math.max(2.0, (2 * math.pi * baseRadius / count) * 0.6);

    for (int i = 0; i < count; i++) {
      final double magnitude = spectrum[i].clamp(0.0, 1.0);
      final double rayLength = magnitude * (maxRadius - baseRadius);
      final double angle = i * angleStep - (math.pi / 2);

      final double xStart = center.dx + math.cos(angle) * baseRadius;
      final double yStart = center.dy + math.sin(angle) * baseRadius;
      final double xEnd = center.dx + math.cos(angle) * (baseRadius + rayLength);
      final double yEnd = center.dy + math.sin(angle) * (baseRadius + rayLength);

      final double t = i / count;
      rayPaint.color = Color.lerp(theme.primary, theme.secondary, (math.sin(t * math.pi) * 1.5).clamp(0.0, 1.0))!;

      canvas.drawLine(Offset(xStart, yStart), Offset(xEnd, yEnd), rayPaint);
    }
  }

  @override
  bool shouldRepaint(covariant CircularPainter oldDelegate) => true;
}
