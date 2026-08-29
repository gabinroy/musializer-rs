import 'package:flutter/material.dart';
import '../models/visualizer_theme.dart';
import '../src/rust/api.dart';

class TrackHeader extends StatelessWidget {
  final MobileTrackInfo? track;
  final VisualizerTheme theme;
  final VoidCallback onPickAudio;

  const TrackHeader({
    super.key,
    required this.track,
    required this.theme,
    required this.onPickAudio,
  });

  @override
  Widget build(BuildContext context) {
    return Container(
      padding: const EdgeInsets.symmetric(horizontal: 20.0, vertical: 12.0),
      child: Row(
        children: [
          // Project App Icon branding
          Container(
            width: 44,
            height: 44,
            decoration: BoxDecoration(
              borderRadius: BorderRadius.circular(12.0),
              boxShadow: [
                BoxShadow(
                  color: theme.glow,
                  blurRadius: 12.0,
                  spreadRadius: 1.0,
                ),
              ],
            ),
            child: ClipRRect(
              borderRadius: BorderRadius.circular(12.0),
              child: Image.asset(
                'assets/icon.png',
                fit: BoxFit.cover,
              ),
            ),
          ),
          const SizedBox(width: 14.0),

          // Track Details
          Expanded(
            child: Column(
              crossAxisAlignment: CrossAxisAlignment.start,
              mainAxisSize: MainAxisSize.min,
              children: [
                Text(
                  track?.title ?? 'No Track Loaded',
                  maxLines: 1,
                  overflow: TextOverflow.ellipsis,
                  style: const TextStyle(
                    color: Colors.white,
                    fontSize: 16.0,
                    fontWeight: FontWeight.w700,
                    letterSpacing: -0.2,
                  ),
                ),
                const SizedBox(height: 2.0),
                Text(
                  track != null
                      ? '${(track!.sampleRate / 1000).toStringAsFixed(1)} kHz • ${track!.channels == 2 ? 'Stereo' : 'Mono'}'
                      : 'Tap to load MP3, FLAC, WAV, AAC',
                  style: TextStyle(
                    color: Colors.white.withValues(alpha: 0.6),
                    fontSize: 12.0,
                    fontWeight: FontWeight.w400,
                  ),
                ),
              ],
            ),
          ),

          // Load File Action Button
          FilledButton.icon(
            onPressed: onPickAudio,
            style: FilledButton.styleFrom(
              backgroundColor: theme.primary.withValues(alpha: 0.2),
              foregroundColor: theme.primary,
              side: BorderSide(color: theme.primary.withValues(alpha: 0.5)),
              padding: const EdgeInsets.symmetric(horizontal: 14.0, vertical: 10.0),
              shape: RoundedRectangleBorder(
                borderRadius: BorderRadius.circular(10.0),
              ),
            ),
            icon: const Icon(Icons.audio_file_rounded, size: 18.0),
            label: const Text(
              'Open',
              style: TextStyle(fontWeight: FontWeight.w600, fontSize: 13.0),
            ),
          ),
        ],
      ),
    );
  }
}
