enum CircleCenterDisplay {
  none('Glow Core'),
  timeElapsed('Elapsed Time'),
  timeRemaining('Remaining Time'),
  trackTitle('Track Title'),
  customCover('Custom Cover Image');

  final String title;
  const CircleCenterDisplay(this.title);
}
