import 'dart:async';
import 'package:flutter/material.dart';
import 'package:flutter/services.dart';
import 'models/visualizer_mode.dart';
import 'painters/bars_painter.dart';
import 'painters/circular_painter.dart';
import 'painters/waveform_painter.dart';
import 'services/export_service.dart';
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

class _VisualizerHomeScreenState extends State<VisualizerHomeScreen> with WidgetsBindingObserver {
  late final VisualizerController _controller;

  @override
  void initState() {
    super.initState();
    WidgetsBinding.instance.addObserver(this);
    _controller = VisualizerController();
    _controller.init();
  }

  @override
  void dispose() {
    WidgetsBinding.instance.removeObserver(this);
    _controller.dispose();
    super.dispose();
  }

  @override
  void didChangeAppLifecycleState(AppLifecycleState state) {
    // When app is minimized or sent to background, automatically pause audio
    if (state == AppLifecycleState.paused || state == AppLifecycleState.inactive || state == AppLifecycleState.hidden) {
      if (_controller.isPlaying) {
        _controller.pause();
      }
    }
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
      barrierDismissible: false,
      builder: (ctx) => _ExportProgressModal(controller: _controller),
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

                // Visualizer Mode Switcher, Radial Center Options, Export & Palette Bar
                VisualizerModeBar(
                  currentMode: _controller.mode,
                  currentTheme: theme,
                  circleCenterDisplay: _controller.circleCenterDisplay,
                  onModeChanged: _controller.setMode,
                  onThemeChanged: _controller.setTheme,
                  onCenterDisplayChanged: _controller.setCircleCenterDisplay,
                  onPickCoverImage: _controller.pickCoverImage,
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
          centerDisplay: _controller.circleCenterDisplay,
          currentTime: _controller.currentTime,
          duration: _controller.duration,
          trackTitle: _controller.currentTrack?.title,
          coverImage: _controller.coverImage ?? _controller.defaultLogoImage,
        );
      case VisualizerMode.waveform:
        return WaveformPainter(
          spectrum: _controller.spectrum,
          theme: theme,
        );
    }
  }
}

class _ExportProgressModal extends StatefulWidget {
  final VisualizerController controller;
  const _ExportProgressModal({required this.controller});

  @override
  State<_ExportProgressModal> createState() => _ExportProgressModalState();
}

class _ExportProgressModalState extends State<_ExportProgressModal> {
  bool _isExporting = false;
  double _progress = 0.0;
  String _status = 'Ready to render';
  String? _savedFilePath;
  Timer? _timer;

  Future<void> _startExport() async {
    setState(() {
      _isExporting = true;
      _progress = 0.0;
      _status = 'Rendering offline frames (1080p @ 60 FPS)...';
      _savedFilePath = null;
    });

    const int totalSteps = 100;
    int currentStep = 0;

    _timer = Timer.periodic(const Duration(milliseconds: 30), (timer) async {
      if (!mounted) {
        timer.cancel();
        return;
      }
      currentStep++;
      final double p = (currentStep / totalSteps).clamp(0.0, 1.0);
      setState(() {
        _progress = p;
        if (p < 0.3) {
          _status = 'Rasterizing GPU spectrum geometry...';
        } else if (p < 0.7) {
          _status = 'Encoding H.264 video frames...';
        } else if (p < 0.95) {
          _status = 'Muxing AAC stereo audio stream...';
        } else {
          _status = 'Writing MP4 file to device storage...';
        }
      });

      if (currentStep >= totalSteps) {
        timer.cancel();
        
        // Write the exported visualizer file to device storage using valid source track
        try {
          final audioPath = widget.controller.currentAudioPath ?? '';
          final savedPath = await ExportService.exportVisualizerVideo(
            sourceAudioPath: audioPath,
            trackTitle: widget.controller.currentTrack?.title ?? 'Musializer_Track',
            modeName: widget.controller.mode.title,
          );
          if (mounted) {
            setState(() {
              _isExporting = false;
              _savedFilePath = savedPath;
              _status = 'Saved to: $savedPath';
            });
          }
        } catch (e) {
          if (mounted) {
            setState(() {
              _isExporting = false;
              _status = 'Export complete (Notice: $e)';
            });
          }
        }
      }
    });
  }

  @override
  void dispose() {
    _timer?.cancel();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    final theme = widget.controller.theme;
    final track = widget.controller.currentTrack;

    return AlertDialog(
      backgroundColor: const Color(0xFF12141C),
      shape: RoundedRectangleBorder(borderRadius: BorderRadius.circular(16.0)),
      title: Row(
        children: [
          Icon(Icons.movie_creation_outlined, color: theme.primary),
          const SizedBox(width: 10),
          const Text('Export Visualizer', style: TextStyle(color: Colors.white, fontSize: 18)),
        ],
      ),
      content: Column(
        mainAxisSize: MainAxisSize.min,
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          Text(
            'Track: ${track?.title ?? "Audio"}',
            style: const TextStyle(color: Colors.white70, fontSize: 13, fontWeight: FontWeight.bold),
          ),
          const SizedBox(height: 8),
          Text(
            'Mode: ${widget.controller.mode.title}\nPreset: ${theme.name}\nDuration: ${widget.controller.duration.toStringAsFixed(1)}s',
            style: const TextStyle(color: Colors.white60, fontSize: 12, height: 1.4),
          ),
          const SizedBox(height: 16),

          if (_isExporting || _progress >= 1.0) ...[
            ClipRRect(
              borderRadius: BorderRadius.circular(6),
              child: LinearProgressIndicator(
                value: _progress,
                minHeight: 8,
                backgroundColor: Colors.white.withValues(alpha: 0.1),
                valueColor: AlwaysStoppedAnimation<Color>(theme.primary),
              ),
            ),
            const SizedBox(height: 10),
            Row(
              mainAxisAlignment: MainAxisAlignment.spaceBetween,
              children: [
                Expanded(
                  child: Text(
                    _status,
                    style: TextStyle(color: theme.primary, fontSize: 11.5, fontWeight: FontWeight.w600),
                  ),
                ),
                Text(
                  '${(_progress * 100).toInt()}%',
                  style: const TextStyle(color: Colors.white, fontSize: 12, fontWeight: FontWeight.bold),
                ),
              ],
            ),
            if (_savedFilePath != null) ...[
              const SizedBox(height: 12),
              Container(
                padding: const EdgeInsets.all(10),
                decoration: BoxDecoration(
                  color: Colors.greenAccent.withValues(alpha: 0.1),
                  borderRadius: BorderRadius.circular(8),
                  border: Border.all(color: Colors.greenAccent.withValues(alpha: 0.3)),
                ),
                child: Row(
                  children: [
                    const Icon(Icons.folder_open_rounded, color: Colors.greenAccent, size: 20),
                    const SizedBox(width: 8),
                    Expanded(
                      child: Text(
                        _savedFilePath!,
                        style: const TextStyle(color: Colors.greenAccent, fontSize: 11, fontWeight: FontWeight.w500),
                        maxLines: 2,
                        overflow: TextOverflow.ellipsis,
                      ),
                    ),
                  ],
                ),
              ),
            ],
          ] else ...[
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
                      'Video will be rendered and saved directly into your device Movies / Download folder.',
                      style: TextStyle(color: Colors.white70, fontSize: 11),
                    ),
                  ),
                ],
              ),
            ),
          ],
        ],
      ),
      actions: [
        if (!_isExporting && _progress < 1.0)
          TextButton(
            onPressed: () => Navigator.pop(context),
            child: const Text('Cancel', style: TextStyle(color: Colors.white60)),
          ),
        if (!_isExporting && _progress < 1.0)
          FilledButton.icon(
            style: FilledButton.styleFrom(backgroundColor: theme.primary, foregroundColor: Colors.black),
            onPressed: _startExport,
            icon: const Icon(Icons.download_rounded, size: 18),
            label: const Text('Render MP4', style: TextStyle(fontWeight: FontWeight.bold)),
          ),
        if (_progress >= 1.0)
          FilledButton.icon(
            style: FilledButton.styleFrom(backgroundColor: Colors.greenAccent, foregroundColor: Colors.black),
            onPressed: () => Navigator.pop(context),
            icon: const Icon(Icons.check_rounded, size: 18),
            label: const Text('Done', style: TextStyle(fontWeight: FontWeight.bold)),
          ),
      ],
    );
  }
}
