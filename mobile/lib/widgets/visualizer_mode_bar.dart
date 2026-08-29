import 'package:flutter/material.dart';
import '../models/circle_center_display.dart';
import '../models/visualizer_mode.dart';
import '../models/visualizer_theme.dart';

class VisualizerModeBar extends StatelessWidget {
  final VisualizerMode currentMode;
  final VisualizerTheme currentTheme;
  final CircleCenterDisplay circleCenterDisplay;
  final ValueChanged<VisualizerMode> onModeChanged;
  final ValueChanged<VisualizerTheme> onThemeChanged;
  final ValueChanged<CircleCenterDisplay> onCenterDisplayChanged;
  final VoidCallback? onPickCoverImage;
  final VoidCallback? onExport;

  const VisualizerModeBar({
    super.key,
    required this.currentMode,
    required this.currentTheme,
    required this.circleCenterDisplay,
    required this.onModeChanged,
    required this.onThemeChanged,
    required this.onCenterDisplayChanged,
    this.onPickCoverImage,
    this.onExport,
  });

  @override
  Widget build(BuildContext context) {
    return Padding(
      padding: const EdgeInsets.symmetric(horizontal: 16.0, vertical: 6.0),
      child: Column(
        mainAxisSize: MainAxisSize.min,
        children: [
          Row(
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

              // Action Buttons: Radial Center Options / Export Video / Palette Picker
              Row(
                mainAxisSize: MainAxisSize.min,
                children: [
                  if (currentMode == VisualizerMode.circular)
                    IconButton(
                      visualDensity: VisualDensity.compact,
                      padding: const EdgeInsets.all(6.0),
                      constraints: const BoxConstraints(minWidth: 36, minHeight: 36),
                      onPressed: () => _showCenterDisplayPicker(context),
                      icon: Icon(Icons.center_focus_strong_rounded, color: currentTheme.secondary, size: 20),
                      tooltip: 'Radial Center Display',
                    ),
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
        ],
      ),
    );
  }

  void _showCenterDisplayPicker(BuildContext context) {
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
              Row(
                mainAxisAlignment: MainAxisAlignment.spaceBetween,
                children: [
                  const Text(
                    'Radial Center Display',
                    style: TextStyle(
                      color: Colors.white,
                      fontSize: 18.0,
                      fontWeight: FontWeight.bold,
                    ),
                  ),
                  if (onPickCoverImage != null)
                    TextButton.icon(
                      style: TextButton.styleFrom(foregroundColor: currentTheme.primary),
                      onPressed: () {
                        Navigator.pop(ctx);
                        onPickCoverImage!();
                      },
                      icon: const Icon(Icons.image_outlined, size: 18),
                      label: const Text('Pick Image'),
                    ),
                ],
              ),
              const SizedBox(height: 12.0),
              ...CircleCenterDisplay.values.map((display) {
                final bool isSelected = display == circleCenterDisplay;
                return ListTile(
                  contentPadding: EdgeInsets.zero,
                  leading: Icon(
                    _getIconForDisplay(display),
                    color: isSelected ? currentTheme.primary : Colors.white60,
                  ),
                  title: Text(
                    display.title,
                    style: TextStyle(
                      color: isSelected ? currentTheme.primary : Colors.white,
                      fontWeight: isSelected ? FontWeight.bold : FontWeight.normal,
                    ),
                  ),
                  trailing: isSelected ? Icon(Icons.check_rounded, color: currentTheme.primary) : null,
                  onTap: () {
                    onCenterDisplayChanged(display);
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

  IconData _getIconForDisplay(CircleCenterDisplay display) {
    switch (display) {
      case CircleCenterDisplay.none:
        return Icons.blur_on_rounded;
      case CircleCenterDisplay.timeElapsed:
        return Icons.timer_outlined;
      case CircleCenterDisplay.timeRemaining:
        return Icons.timelapse_rounded;
      case CircleCenterDisplay.trackTitle:
        return Icons.music_note_rounded;
      case CircleCenterDisplay.customCover:
        return Icons.image_rounded;
    }
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
