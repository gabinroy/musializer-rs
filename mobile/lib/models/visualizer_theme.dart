import 'package:flutter/material.dart';

class VisualizerTheme {
  final String name;
  final Color primary;
  final Color secondary;
  final Color accent;
  final Color glow;
  final Color background;
  final LinearGradient barGradient;

  const VisualizerTheme({
    required this.name,
    required this.primary,
    required this.secondary,
    required this.accent,
    required this.glow,
    required this.background,
    required this.barGradient,
  });

  static const List<VisualizerTheme> presets = [
    VisualizerTheme(
      name: 'Cyberpunk Neon',
      primary: Color(0xFF00F0FF),
      secondary: Color(0xFFFF007F),
      accent: Color(0xFF7928CA),
      glow: Color(0x6600F0FF),
      background: Color(0xFF0A0B10),
      barGradient: LinearGradient(
        begin: Alignment.bottomCenter,
        end: Alignment.topCenter,
        colors: [Color(0xFF00F0FF), Color(0xFF7928CA), Color(0xFFFF007F)],
      ),
    ),
    VisualizerTheme(
      name: 'Electric Cyan',
      primary: Color(0xFF00E5FF),
      secondary: Color(0xFF2979FF),
      accent: Color(0xFF00B0FF),
      glow: Color(0x6600E5FF),
      background: Color(0xFF060D1A),
      barGradient: LinearGradient(
        begin: Alignment.bottomCenter,
        end: Alignment.topCenter,
        colors: [Color(0xFF00E5FF), Color(0xFF2979FF), Color(0xFF651FFF)],
      ),
    ),
    VisualizerTheme(
      name: 'Solar Flare',
      primary: Color(0xFFFF9100),
      secondary: Color(0xFFFF1744),
      accent: Color(0xFFFFEA00),
      glow: Color(0x66FF9100),
      background: Color(0xFF140804),
      barGradient: LinearGradient(
        begin: Alignment.bottomCenter,
        end: Alignment.topCenter,
        colors: [Color(0xFFFFEA00), Color(0xFFFF9100), Color(0xFFFF1744)],
      ),
    ),
    VisualizerTheme(
      name: 'Matrix Emerald',
      primary: Color(0xFF00E676),
      secondary: Color(0xFF00B0FF),
      accent: Color(0xFF76FF03),
      glow: Color(0x6600E676),
      background: Color(0xFF031008),
      barGradient: LinearGradient(
        begin: Alignment.bottomCenter,
        end: Alignment.topCenter,
        colors: [Color(0xFF76FF03), Color(0xFF00E676), Color(0xFF00B0FF)],
      ),
    ),
  ];
}
