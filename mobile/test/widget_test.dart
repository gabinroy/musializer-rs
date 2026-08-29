import 'package:flutter_test/flutter_test.dart';
import 'package:mobile/models/visualizer_mode.dart';
import 'package:mobile/models/visualizer_theme.dart';

void main() {
  test('Visualizer presets loaded properly', () {
    expect(VisualizerTheme.presets.isNotEmpty, true);
    expect(VisualizerMode.values.length, 3);
  });
}
