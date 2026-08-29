import 'dart:math' as math;
import 'dart:typed_data';
import 'dart:ui' as ui;
import 'package:flutter/material.dart';
import '../models/circle_center_display.dart';
import '../models/visualizer_theme.dart';

class CircularPainter extends CustomPainter {
  final Float32List spectrum;
  final VisualizerTheme theme;
  final CircleCenterDisplay centerDisplay;
  final double currentTime;
  final double duration;
  final String? trackTitle;
  final ui.Image? coverImage;

  CircularPainter({
    required this.spectrum,
    required this.theme,
    this.centerDisplay = CircleCenterDisplay.timeElapsed,
    this.currentTime = 0.0,
    this.duration = 0.0,
    this.trackTitle,
    this.coverImage,
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
    final double baseRadius = maxRadius * 0.45 + (bassEnergy * 20.0);

    // Inner Glow Circle
    final Paint centerGlow = Paint()
      ..color = theme.glow.withValues(alpha: 0.25 + (bassEnergy * 0.35))
      ..maskFilter = const MaskFilter.blur(BlurStyle.normal, 24.0);
    canvas.drawCircle(center, baseRadius * 0.95, centerGlow);

    // Inner Core Circle Background
    final Paint corePaint = Paint()
      ..shader = RadialGradient(
        colors: [theme.primary.withValues(alpha: 0.8), const Color(0xFF0A0D16)],
      ).createShader(Rect.fromCircle(center: center, radius: baseRadius));
    canvas.drawCircle(center, baseRadius, corePaint);

    // Render Center Display (Cover Image or Dynamic Text)
    if (centerDisplay == CircleCenterDisplay.customCover && coverImage != null) {
      canvas.save();
      final Path clipPath = Path()..addOval(Rect.fromCircle(center: center, radius: baseRadius * 0.92));
      canvas.clipPath(clipPath);
      paintImage(
        canvas: canvas,
        rect: Rect.fromCircle(center: center, radius: baseRadius * 0.92),
        image: coverImage!,
        fit: BoxFit.cover,
      );
      canvas.restore();
    } else if (centerDisplay == CircleCenterDisplay.timeElapsed) {
      _drawText(
        canvas,
        center,
        _formatTime(currentTime),
        _formatTime(duration),
        isSecondarySubtitle: true,
      );
    } else if (centerDisplay == CircleCenterDisplay.timeRemaining) {
      final double remaining = math.max(0.0, duration - currentTime);
      _drawText(
        canvas,
        center,
        '-${_formatTime(remaining)}',
        'REMAINING',
        isSecondarySubtitle: true,
      );
    } else if (centerDisplay == CircleCenterDisplay.trackTitle) {
      final String displayTitle = (trackTitle != null && trackTitle!.isNotEmpty)
          ? (trackTitle!.length > 14 ? '${trackTitle!.substring(0, 12)}...' : trackTitle!)
          : 'Musializer';
      _drawText(
        canvas,
        center,
        displayTitle,
        'NOW PLAYING',
        isSecondarySubtitle: true,
        fontSize: 16.0,
      );
    }

    // Outer glow border around center
    final Paint borderPaint = Paint()
      ..style = PaintingStyle.stroke
      ..strokeWidth = 2.5
      ..shader = theme.barGradient.createShader(Rect.fromCircle(center: center, radius: baseRadius));
    canvas.drawCircle(center, baseRadius, borderPaint);

    if (spectrum.isEmpty) return;

    final int count = spectrum.length;
    final double angleStep = (2 * math.pi) / count;

    final Paint rayPaint = Paint()
      ..style = PaintingStyle.stroke
      ..strokeCap = StrokeCap.round
      ..strokeWidth = math.max(2.5, (2 * math.pi * baseRadius / count) * 0.6);

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

  void _drawText(
    Canvas canvas,
    Offset center,
    String mainText,
    String subText, {
    bool isSecondarySubtitle = false,
    double fontSize = 22.0,
  }) {
    final TextPainter tpMain = TextPainter(
      text: TextSpan(
        text: mainText,
        style: TextStyle(
          color: Colors.white,
          fontSize: fontSize,
          fontWeight: FontWeight.bold,
          letterSpacing: 0.5,
        ),
      ),
      textAlign: TextAlign.center,
      textDirection: TextDirection.ltr,
    )..layout(maxWidth: 160);

    final TextPainter tpSub = TextPainter(
      text: TextSpan(
        text: subText,
        style: TextStyle(
          color: Colors.white.withValues(alpha: 0.6),
          fontSize: 11.0,
          fontWeight: FontWeight.w600,
          letterSpacing: 1.0,
        ),
      ),
      textAlign: TextAlign.center,
      textDirection: TextDirection.ltr,
    )..layout(maxWidth: 160);

    tpMain.paint(canvas, Offset(center.dx - tpMain.width / 2, center.dy - 14.0));
    tpSub.paint(canvas, Offset(center.dx - tpSub.width / 2, center.dy + 12.0));
  }

  String _formatTime(double seconds) {
    final int totalSecs = math.max(0, seconds.toInt());
    final int mins = totalSecs ~/ 60;
    final int secs = totalSecs % 60;
    return '${mins.toString().padLeft(2, '0')}:${secs.toString().padLeft(2, '0')}';
  }

  @override
  bool shouldRepaint(covariant CircularPainter oldDelegate) => true;
}
