/// Supported visualizer rendering modes for mobile GPU canvas.
enum VisualizerMode {
  /// Vertical frequency spectrum bars with floating peak caps.
  spectrumBars('Spectrum Bars', 'Vertical frequency bars with peak-decay caps'),

  /// 360-degree radial visualizer with pulsating inner core.
  circular('Radial Pulse', 'Circular 360-degree bass-reactive visualizer'),

  /// Smooth cubic bezier oscilloscopic waveform.
  waveform('Smooth Wave', 'Fluid cubic bezier oscilloscope waveform');

  /// Human-readable display label.
  final String title;

  /// Mode description explaining the visual effect.
  final String description;

  const VisualizerMode(this.title, this.description);
}
