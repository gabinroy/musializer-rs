enum VisualizerMode {
  spectrumBars('Spectrum Bars', 'Vertical frequency bars with peak-decay caps'),
  circular('Radial Pulse', 'Circular 360-degree bass-reactive visualizer'),
  waveform('Smooth Wave', 'Fluid cubic bezier oscilloscope waveform');

  final String title;
  final String description;
  const VisualizerMode(this.title, this.description);
}
