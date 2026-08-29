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

                // Visualizer Mode Switcher & Palette Bar
                VisualizerModeBar(
                  currentMode: _controller.mode,
                  currentTheme: theme,
                  onModeChanged: _controller.setMode,
                  onThemeChanged: _controller.setTheme,
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
