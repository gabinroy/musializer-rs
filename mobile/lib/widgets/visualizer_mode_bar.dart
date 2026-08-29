import 'package:flutter/material.dart';
import '../models/visualizer_mode.dart';
import '../models/visualizer_theme.dart';

class VisualizerModeBar extends StatelessWidget {
  final VisualizerMode currentMode;
  final VisualizerTheme currentTheme;
  final ValueChanged<VisualizerMode> onModeChanged;
  final ValueChanged<VisualizerTheme> onThemeChanged;
  final VoidCallback? onExport;

  const VisualizerModeBar({
    super.key,
    required this.currentMode,
    required this.currentTheme,
    required this.onModeChanged,
    required this.onThemeChanged,
    this.onExport,
  });

  @override
  Widget build(BuildContext context) {
    return Padding(
      padding: const EdgeInsets.symmetric(horizontal: 16.0, vertical: 6.0),
      child: Row(
        children: [
          // Flexible Segmented Mode Switcher
          Expanded(
            child: Container(
              padding: const EdgeInsets.all(3.0),
              decoration: BoxDecoration(
                color: Colors.white.withValues(alpha: 0.05),
                borderRadius: BorderRadius.circular(12.0),
                border: Border.all(color: Colors.white.withValues(alpha: 0.1)),
              ),
              child: Row(
                children: VisualizerMode.values.map((mode) {
                  final bool isSelected = mode == currentMode;
                  return Expanded(
                    child: GestureDetector(
                      onTap: () => onModeChanged(mode),
                      child: AnimatedContainer(
                        duration: const Duration(milliseconds: 200),
                        padding: const EdgeInsets.symmetric(vertical: 7.0),
                        alignment: Alignment.center,
                        decoration: BoxDecoration(
                          color: isSelected ? currentTheme.primary : Colors.transparent,
                          borderRadius: BorderRadius.circular(8.0),
                        ),
                        child: Text(
                          mode.title,
                          maxLines: 1,
                          overflow: TextOverflow.ellipsis,
                          style: TextStyle(
                            color: isSelected ? Colors.black : Colors.white70,
                            fontWeight: isSelected ? FontWeight.w700 : FontWeight.w500,
                            fontSize: 11.5,
                          ),
                        ),
                      ),
                    ),
                  );
                }).toList(),
              ),
            ),
          ),

          const SizedBox(width: 8.0),

          // Action Buttons: Export Video & Palette Picker
          Row(
            mainAxisSize: MainAxisSize.min,
            children: [
              if (onExport != null)
                IconButton(
                  visualDensity: VisualDensity.compact,
                  padding: const EdgeInsets.all(6.0),
                  constraints: const BoxConstraints(minWidth: 36, minHeight: 36),
                  onPressed: onExport,
                  icon: Icon(Icons.movie_creation_outlined, color: currentTheme.primary, size: 20),
                  tooltip: 'Export Visualizer Video',
                ),
              IconButton(
                visualDensity: VisualDensity.compact,
                padding: const EdgeInsets.all(6.0),
                constraints: const BoxConstraints(minWidth: 36, minHeight: 36),
                onPressed: () => _showThemePicker(context),
                icon: Icon(Icons.palette_rounded, color: currentTheme.primary, size: 20),
                tooltip: 'Change Color Palette',
              ),
            ],
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
