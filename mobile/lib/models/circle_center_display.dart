/// Overlay display mode rendered in the center of the radial/circular visualizer.
enum CircleCenterDisplay {
  /// Pure animated glowing radial core.
  none('Glow Core'),

  /// Displays current track elapsed timestamp (e.g. `01:23`).
  timeElapsed('Elapsed Time'),

  /// Displays countdown of remaining audio duration (e.g. `-02:45`).
  timeRemaining('Remaining Time'),

  /// Displays scrolling or centered track title.
  trackTitle('Track Title'),

  /// Displays circular clipped user album art or cover image.
  customCover('Custom Cover Image');

  /// UI title label.
  final String title;

  const CircleCenterDisplay(this.title);
}
