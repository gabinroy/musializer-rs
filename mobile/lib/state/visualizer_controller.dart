import 'dart:async';
import 'dart:math' as math;
import 'package:file_picker/file_picker.dart';
import 'package:flutter/foundation.dart';
import 'package:flutter/scheduler.dart';
import '../models/visualizer_mode.dart';
import '../models/visualizer_theme.dart';
import '../src/rust/api.dart' as rust_api;

class VisualizerController extends ChangeNotifier {
  static const int defaultFftSize = 2048;
  static const int defaultNumBands = 64;

  bool _initialized = false;
  bool get isInitialized => _initialized;

  VisualizerMode _mode = VisualizerMode.spectrumBars;
  VisualizerMode get mode => _mode;

  VisualizerTheme _theme = VisualizerTheme.presets.first;
  VisualizerTheme get theme => _theme;

  rust_api.MobileTrackInfo? _currentTrack;
  rust_api.MobileTrackInfo? get currentTrack => _currentTrack;

  bool _isPlaying = false;
  bool get isPlaying => _isPlaying;

  double _currentTime = 0.0;
  double get currentTime => _currentTime;

  double _duration = 0.0;
  double get duration => _duration;

  double _volume = 1.0;
  double get volume => _volume;

  double _gainMultiplier = 1.0;
  double get gainMultiplier => _gainMultiplier;

  Float32List _spectrum = Float32List(defaultNumBands);
  Float32List get spectrum => _spectrum;

  List<double> _peaks = List.filled(defaultNumBands, 0.0);
  List<double> get peaks => _peaks;

  Ticker? _ticker;
  Duration _lastTick = Duration.zero;

  Future<void> init() async {
    if (_initialized) return;
    try {
      await rust_api.initEngine(
        fftSize: BigInt.from(defaultFftSize),
        numBands: BigInt.from(defaultNumBands),
      );
      _initialized = true;

      _ticker = Ticker(_onTick);
      _ticker?.start();
      notifyListeners();
    } catch (e) {
      debugPrint('Error initializing Rust engine: $e');
    }
  }

  void setMode(VisualizerMode newMode) {
    _mode = newMode;
    notifyListeners();
  }

  void setTheme(VisualizerTheme newTheme) {
    _theme = newTheme;
    notifyListeners();
  }

  Future<void> pickAndLoadAudio() async {
    try {
      final FilePickerResult? result = await FilePicker.platform.pickFiles(
        type: FileType.custom,
        allowedExtensions: ['mp3', 'wav', 'flac', 'ogg', 'aac', 'm4a'],
      );

      if (result != null && result.files.isNotEmpty) {
        final path = result.files.single.path;
        if (path != null) {
          final track = await rust_api.loadAudioFile(path: path);
          _currentTrack = track;
          _duration = track.durationSeconds;
          _currentTime = 0.0;
          _isPlaying = true;
          await rust_api.play();
          notifyListeners();
        }
      }
    } catch (e) {
      debugPrint('Error loading audio file: $e');
    }
  }

  Future<void> togglePlayPause() async {
    await rust_api.togglePlayPause();
    _isPlaying = await rust_api.isPlaying();
    notifyListeners();
  }

  Future<void> seek(double seconds) async {
    await rust_api.seekSeconds(seconds: seconds);
    _currentTime = seconds;
    notifyListeners();
  }

  Future<void> setVolume(double val) async {
    _volume = val.clamp(0.0, 2.0);
    await rust_api.setVolume(volume: _volume);
    notifyListeners();
  }

  Future<void> setGainMultiplier(double val) async {
    _gainMultiplier = val.clamp(0.1, 4.0);
    await rust_api.setGainMultiplier(gain: _gainMultiplier);
    notifyListeners();
  }

  void _onTick(Duration elapsed) async {
    if (!_initialized) return;

    final double dt = _lastTick == Duration.zero
        ? 0.016
        : (elapsed - _lastTick).inMicroseconds / 1000000.0;
    _lastTick = elapsed;

    final playing = await rust_api.isPlaying();
    if (playing != _isPlaying) {
      _isPlaying = playing;
    }

    if (_isPlaying) {
      _currentTime = await rust_api.currentTime();
    }

    final newSpectrum = await rust_api.getSpectrum(dt: dt);
    if (newSpectrum.isNotEmpty) {
      _spectrum = newSpectrum;

      // Update Peak-Hold Decay
      if (_peaks.length != newSpectrum.length) {
        _peaks = List.filled(newSpectrum.length, 0.0);
      }
      for (int i = 0; i < newSpectrum.length; i++) {
        if (newSpectrum[i] > _peaks[i]) {
          _peaks[i] = newSpectrum[i];
        } else {
          _peaks[i] = math.max(0.0, _peaks[i] - 0.4 * dt);
        }
      }
    }

    notifyListeners();
  }

  @override
  void dispose() {
    _ticker?.dispose();
    super.dispose();
  }
}
