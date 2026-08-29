import 'package:flutter/material.dart';
import '../models/visualizer_mode.dart';
import '../models/visualizer_theme.dart';

class VisualizerModeBar extends StatelessWidget {
  final VisualizerMode currentMode;
  final VisualizerTheme currentTheme;
  final ValueChanged<VisualizerMode> onModeChanged;
  final ValueChanged<VisualizerTheme> onThemeChanged;

  const VisualizerModeBar({
    super.key,
    required this.currentMode,
    required this.currentTheme,
    required this.onModeChanged,
    required this.onThemeChanged,
  });

  @override
  Widget build(BuildContext context) {
    return Padding(
      padding: const EdgeInsets.symmetric(horizontal: 20.0, vertical: 8.0),
      child: Row(
        mainAxisAlignment: MainAxisAlignment.spaceBetween,
        children: [
          // Mode Switcher Segmented Control
          Container(
            padding: const EdgeInsets.all(4.0),
            decoration: BoxDecoration(
              color: Colors.white.withValues(alpha: 0.05),
              borderRadius: BorderRadius.circular(12.0),
              border: Border.all(color: Colors.white.withValues(alpha: 0.1)),
            ),
            child: Row(
              children: VisualizerMode.values.map((mode) {
                final bool isSelected = mode == currentMode;
                return GestureDetector(
                  onTap: () => onModeChanged(mode),
                  child: AnimatedContainer(
                    duration: const Duration(milliseconds: 200),
                    padding: const EdgeInsets.symmetric(horizontal: 12.0, vertical: 6.0),
                    decoration: BoxDecoration(
                      color: isSelected ? currentTheme.primary : Colors.transparent,
                      borderRadius: BorderRadius.circular(8.0),
                    ),
                    child: Text(
                      mode.title,
                      style: TextStyle(
                        color: isSelected ? Colors.black : Colors.white70,
                        fontWeight: isSelected ? FontWeight.w700 : FontWeight.w500,
                        fontSize: 12.0,
                      ),
                    ),
                  ),
                );
              }).toList(),
            ),
          ),

          // Palette Picker Icon Button
          IconButton(
            onPressed: () => _showThemePicker(context),
            icon: Icon(Icons.palette_rounded, color: currentTheme.primary),
            tooltip: 'Change Color Palette',
          ),
        ],
      ),
    );
  }

  void _showThemePicker(BuildContext context) {
    showModalBottomSheet(
      context: context,
      backgroundColor: const Color(0xFF12141C),
      shape: const RoundedRectangleBorder(
        borderRadius: BorderRadius.vertical(top: Radius.circular(20.0)),
      ),
      builder: (ctx) {
        return Padding(
          padding: const EdgeInsets.all(20.0),
          child: Column(
            mainAxisSize: MainAxisSize.min,
            crossAxisAlignment: CrossAxisAlignment.start,
            children: [
              const Text(
                'Color Palettes',
                style: TextStyle(
                  color: Colors.white,
                  fontSize: 18.0,
                  fontWeight: FontWeight.bold,
                ),
              ),
              const SizedBox(height: 16.0),
              ...VisualizerTheme.presets.map((theme) {
                final bool isSelected = theme.name == currentTheme.name;
                return ListTile(
                  contentPadding: EdgeInsets.zero,
                  leading: Container(
                    width: 24,
                    height: 24,
                    decoration: BoxDecoration(
                      shape: BoxShape.circle,
                      gradient: theme.barGradient,
                    ),
                  ),
                  title: Text(
                    theme.name,
                    style: TextStyle(
                      color: isSelected ? theme.primary : Colors.white,
                      fontWeight: isSelected ? FontWeight.bold : FontWeight.normal,
                    ),
                  ),
                  trailing: isSelected ? Icon(Icons.check_rounded, color: theme.primary) : null,
                  onTap: () {
                    onThemeChanged(theme);
                    Navigator.pop(ctx);
                  },
                );
              }),
            ],
          ),
        );
      },
    );
  }
}
