import 'package:flutter/material.dart';
import '../models/visualizer_theme.dart';

class PlaybackControls extends StatelessWidget {
  final bool isPlaying;
  final double currentTime;
  final double duration;
  final double volume;
  final double gainMultiplier;
  final VisualizerTheme theme;
  final VoidCallback onTogglePlay;
  final ValueChanged<double> onSeek;
  final ValueChanged<double> onVolumeChanged;
  final ValueChanged<double> onGainChanged;

  const PlaybackControls({
    super.key,
    required this.isPlaying,
    required this.currentTime,
    required this.duration,
    required this.volume,
    required this.gainMultiplier,
    required this.theme,
    required this.onTogglePlay,
    required this.onSeek,
    required this.onVolumeChanged,
    required this.onGainChanged,
  });

  String _formatTime(double seconds) {
    if (seconds.isNaN || seconds.isInfinite || seconds < 0) return '00:00';
    final int mins = seconds ~/ 60;
    final int secs = (seconds % 60).toInt();
    return '${mins.toString().padLeft(2, '0')}:${secs.toString().padLeft(2, '0')}';
  }

  @override
  Widget build(BuildContext context) {
    final double progress = duration > 0 ? (currentTime / duration).clamp(0.0, 1.0) : 0.0;

    return Container(
      padding: const EdgeInsets.symmetric(horizontal: 20.0, vertical: 12.0),
      decoration: BoxDecoration(
        color: const Color(0xFF0F111A).withValues(alpha: 0.8),
        borderRadius: const BorderRadius.vertical(top: Radius.circular(24.0)),
        border: Border(
          top: BorderSide(color: Colors.white.withValues(alpha: 0.08)),
        ),
      ),
      child: Column(
        mainAxisSize: MainAxisSize.min,
        children: [
          // Seek Timeline Slider
          SliderTheme(
            data: SliderTheme.of(context).copyWith(
              trackHeight: 4.0,
              thumbShape: const RoundSliderThumbShape(enabledThumbRadius: 6.0),
              overlayShape: const RoundSliderOverlayShape(overlayRadius: 14.0),
              activeTrackColor: theme.primary,
              inactiveTrackColor: Colors.white.withValues(alpha: 0.1),
              thumbColor: theme.primary,
              overlayColor: theme.primary.withValues(alpha: 0.2),
            ),
            child: Slider(
              value: progress,
              onChanged: (val) {
                if (duration > 0) {
                  onSeek(val * duration);
                }
              },
            ),
          ),

          // Timestamps (Elapsed vs Remaining)
          Padding(
            padding: const EdgeInsets.symmetric(horizontal: 16.0),
            child: Row(
              mainAxisAlignment: MainAxisAlignment.spaceBetween,
              children: [
                Text(
                  _formatTime(currentTime),
                  style: TextStyle(color: Colors.white.withValues(alpha: 0.6), fontSize: 12.0),
                ),
                Text(
                  _formatTime(duration),
                  style: TextStyle(color: Colors.white.withValues(alpha: 0.6), fontSize: 12.0),
                ),
              ],
            ),
          ),

          const SizedBox(height: 8.0),

          // Core Playback Buttons
          Row(
            mainAxisAlignment: MainAxisAlignment.spaceEvenly,
            children: [
              // Rewind 5s
              IconButton(
                onPressed: () => onSeek((currentTime - 5.0).clamp(0.0, duration)),
                icon: const Icon(Icons.replay_5_rounded, color: Colors.white70, size: 28.0),
              ),

              // Play / Pause Master Button
              GestureDetector(
                onTap: onTogglePlay,
                child: Container(
                  width: 64,
                  height: 64,
                  decoration: BoxDecoration(
                    shape: BoxShape.circle,
                    gradient: theme.barGradient,
                    boxShadow: [
                      BoxShadow(
                        color: theme.glow,
                        blurRadius: 18.0,
                        spreadRadius: 2.0,
                      ),
                    ],
                  ),
                  child: Icon(
                    isPlaying ? Icons.pause_rounded : Icons.play_arrow_rounded,
                    color: Colors.black,
                    size: 36.0,
                  ),
                ),
              ),

              // Forward 5s
              IconButton(
                onPressed: () => onSeek((currentTime + 5.0).clamp(0.0, duration)),
                icon: const Icon(Icons.forward_5_rounded, color: Colors.white70, size: 28.0),
              ),
            ],
          ),

          const SizedBox(height: 12.0),

          // Visualizer Boost & Volume Row
          Row(
            children: [
              // Gain Multiplier (Visualizer Sensitivity)
              const Icon(Icons.equalizer_rounded, size: 18.0, color: Colors.white54),
              const SizedBox(width: 6.0),
              Text(
                'Gain',
                style: TextStyle(color: Colors.white.withValues(alpha: 0.6), fontSize: 11.0),
              ),
              Expanded(
                child: SliderTheme(
                  data: SliderTheme.of(context).copyWith(
                    trackHeight: 2.0,
                    thumbShape: const RoundSliderThumbShape(enabledThumbRadius: 4.0),
                    activeTrackColor: theme.secondary,
                    inactiveTrackColor: Colors.white.withValues(alpha: 0.08),
                    thumbColor: theme.secondary,
                  ),
                  child: Slider(
                    value: gainMultiplier,
                    min: 0.2,
                    max: 3.0,
                    onChanged: onGainChanged,
                  ),
              ),
              ),

              // Volume Control
              const SizedBox(width: 8.0),
              const Icon(Icons.volume_up_rounded, size: 18.0, color: Colors.white54),
              Expanded(
                child: SliderTheme(
                  data: SliderTheme.of(context).copyWith(
                    trackHeight: 2.0,
                    thumbShape: const RoundSliderThumbShape(enabledThumbRadius: 4.0),
                    activeTrackColor: theme.primary,
                    inactiveTrackColor: Colors.white.withValues(alpha: 0.08),
                    thumbColor: theme.primary,
                  ),
                  child: Slider(
                    value: volume,
                    min: 0.0,
                    max: 1.5,
                    onChanged: onVolumeChanged,
                  ),
                ),
              ),
            ],
          ),
        ],
      ),
    );
  }
}
