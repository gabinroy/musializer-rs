import 'package:flutter/material.dart';
import 'package:flutter/services.dart';
import 'models/visualizer_mode.dart';
import 'painters/bars_painter.dart';
import 'painters/circular_painter.dart';
import 'painters/waveform_painter.dart';
import 'src/rust/frb_generated.dart';
import 'state/visualizer_controller.dart';
import 'widgets/playback_controls.dart';
import 'widgets/track_header.dart';
import 'widgets/visualizer_mode_bar.dart';

Future<void> main() async {
  WidgetsFlutterBinding.ensureInitialized();
  
  // High-performance edge-to-edge styling
  SystemChrome.setSystemUIOverlayStyle(
    const SystemUiOverlayStyle(
      statusBarColor: Colors.transparent,
      statusBarIconBrightness: Brightness.light,
      systemNavigationBarColor: Color(0xFF0A0B10),
      systemNavigationBarIconBrightness: Brightness.light,
    ),
  );

  await RustLib.init();
  runApp(const MusializerApp());
}

class MusializerApp extends StatelessWidget {
  const MusializerApp({super.key});

  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      title: 'Musializer-RS Mobile',
      debugShowCheckedModeBanner: false,
      theme: ThemeData.dark().copyWith(
        scaffoldBackgroundColor: const Color(0xFF0A0B10),
      ),
      home: const VisualizerHomeScreen(),
    );
  }
}

class VisualizerHomeScreen extends StatefulWidget {
  const VisualizerHomeScreen({super.key});

  @override
  State<VisualizerHomeScreen> createState() => _VisualizerHomeScreenState();
}

class _VisualizerHomeScreenState extends State<VisualizerHomeScreen> {
  late final VisualizerController _controller;

  @override
  void initState() {
    super.initState();
    _controller = VisualizerController();
    _controller.init();
  }

  @override
  void dispose() {
    _controller.dispose();
    super.dispose();
  }

  void _showExportDialog(BuildContext context) {
    if (_controller.currentTrack == null) {
      ScaffoldMessenger.of(context).showSnackBar(
        const SnackBar(
          content: Text('Please load an audio file first before exporting!'),
          backgroundColor: Color(0xFF1E2435),
        ),
      );
      return;
    }

    showDialog(
      context: context,
      builder: (ctx) => AlertDialog(
        backgroundColor: const Color(0xFF12141C),
        shape: RoundedRectangleBorder(borderRadius: BorderRadius.circular(16.0)),
        title: Row(
          children: [
            Icon(Icons.movie_creation_outlined, color: _controller.theme.primary),
            const SizedBox(width: 10),
            const Text('Export Visualizer', style: TextStyle(color: Colors.white, fontSize: 18)),
          ],
        ),
        content: Column(
          mainAxisSize: MainAxisSize.min,
          crossAxisAlignment: CrossAxisAlignment.start,
          children: [
            Text(
              'Track: ${_controller.currentTrack?.title ?? "Audio"}',
              style: const TextStyle(color: Colors.white70, fontSize: 13, fontWeight: FontWeight.bold),
            ),
            const SizedBox(height: 12),
            Text(
              'Mode: ${_controller.mode.title}\nPreset: ${_controller.theme.name}\nDuration: ${_controller.duration.toStringAsFixed(1)}s',
              style: const TextStyle(color: Colors.white60, fontSize: 12, height: 1.4),
            ),
            const SizedBox(height: 16),
            Container(
              padding: const EdgeInsets.all(12),
              decoration: BoxDecoration(
                color: Colors.white.withValues(alpha: 0.05),
                borderRadius: BorderRadius.circular(8),
              ),
              child: const Row(
                children: [
                  Icon(Icons.info_outline_rounded, color: Colors.cyanAccent, size: 18),
                  SizedBox(width: 8),
                  Expanded(
                    child: Text(
                      'Direct hardware video rendering on mobile will save MP4 to your device gallery.',
                      style: TextStyle(color: Colors.white70, fontSize: 11),
                    ),
                  ),
                ],
              ),
            ),
          ],
        ),
        actions: [
          TextButton(
            onPressed: () => Navigator.pop(ctx),
            child: const Text('Cancel', style: TextStyle(color: Colors.white60)),
          ),
          FilledButton.icon(
            style: FilledButton.styleFrom(backgroundColor: _controller.theme.primary, foregroundColor: Colors.black),
            onPressed: () {
              Navigator.pop(ctx);
              ScaffoldMessenger.of(context).showSnackBar(
                SnackBar(
                  content: Text('Exporting ${_controller.mode.title} MP4 video...'),
                  backgroundColor: _controller.theme.primary.withValues(alpha: 0.8),
                ),
              );
            },
            icon: const Icon(Icons.download_rounded, size: 18),
            label: const Text('Render MP4', style: TextStyle(fontWeight: FontWeight.bold)),
          ),
        ],
      ),
    );
  }

  @override
  Widget build(BuildContext context) {
    return ListenableBuilder(
      listenable: _controller,
      builder: (context, _) {
        final theme = _controller.theme;

        return Scaffold(
          backgroundColor: theme.background,
          body: SafeArea(
            child: Column(
              children: [
                // Top Track Branding Header
                TrackHeader(
                  track: _controller.currentTrack,
                  theme: theme,
                  onPickAudio: _controller.pickAndLoadAudio,
                ),

                // Visualizer Mode Switcher, Export & Palette Bar
                VisualizerModeBar(
                  currentMode: _controller.mode,
                  currentTheme: theme,
                  onModeChanged: _controller.setMode,
                  onThemeChanged: _controller.setTheme,
                  onExport: () => _showExportDialog(context),
                ),

                // Main 120 FPS Visualizer Canvas
                Expanded(
                  child: Padding(
                    padding: const EdgeInsets.symmetric(horizontal: 16.0, vertical: 8.0),
                    child: ClipRRect(
                      borderRadius: BorderRadius.circular(20.0),
                      child: Container(
                        decoration: BoxDecoration(
                          color: Colors.black.withValues(alpha: 0.3),
                          borderRadius: BorderRadius.circular(20.0),
                          border: Border.all(
                            color: Colors.white.withValues(alpha: 0.05),
                          ),
                        ),
                        child: CustomPaint(
                          painter: _buildPainter(theme),
                          size: Size.infinite,
                        ),
                      ),
                    ),
                  ),
                ),

                // Playback Controls (Seek timeline, volume, play/pause)
                PlaybackControls(
                  isPlaying: _controller.isPlaying,
                  currentTime: _controller.currentTime,
                  duration: _controller.duration,
                  volume: _controller.volume,
                  gainMultiplier: _controller.gainMultiplier,
                  theme: theme,
                  onTogglePlay: _controller.togglePlayPause,
                  onSeek: _controller.seek,
                  onVolumeChanged: _controller.setVolume,
                  onGainChanged: _controller.setGainMultiplier,
                ),
              ],
            ),
          ),
        );
      },
    );
  }

  CustomPainter _buildPainter(theme) {
    switch (_controller.mode) {
      case VisualizerMode.spectrumBars:
        return BarsPainter(
          spectrum: _controller.spectrum,
          peaks: _controller.peaks,
          theme: theme,
        );
      case VisualizerMode.circular:
        return CircularPainter(
          spectrum: _controller.spectrum,
          theme: theme,
        );
      case VisualizerMode.waveform:
        return WaveformPainter(
          spectrum: _controller.spectrum,
          theme: theme,
        );
    }
  }
}
