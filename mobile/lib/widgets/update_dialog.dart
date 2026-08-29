import 'dart:io';
import 'package:flutter/material.dart';
import '../models/visualizer_theme.dart';
import '../services/update_service.dart';

class UpdateDialog extends StatefulWidget {
  final AppReleaseInfo release;
  final VisualizerTheme theme;

  const UpdateDialog({
    super.key,
    required this.release,
    required this.theme,
  });

  static Future<void> showIfAvailable(
    BuildContext context,
    VisualizerTheme theme,
  ) async {
    final release = await UpdateService.checkForUpdates();
    if (release != null && context.mounted) {
      showDialog(
        context: context,
        barrierDismissible: false,
        builder: (ctx) => UpdateDialog(release: release, theme: theme),
      );
    }
  }

  @override
  State<UpdateDialog> createState() => _UpdateDialogState();
}

class _UpdateDialogState extends State<UpdateDialog> {
  bool _isDownloading = false;
  double _progress = 0.0;
  String _status = 'Ready to install';

  Future<void> _startUpdate() async {
    setState(() {
      _isDownloading = true;
      _progress = 0.0;
      _status = 'Connecting to server...';
    });

    try {
      await UpdateService.downloadAndInstallUpdate(
        release: widget.release,
        onProgress: (progress, status) {
          if (mounted) {
            setState(() {
              _progress = progress;
              _status = status;
            });
          }
        },
      );

      if (mounted) {
        setState(() {
          _isDownloading = false;
          _status = 'Opening installer...';
        });
      }
    } catch (e) {
      if (mounted) {
        setState(() {
          _isDownloading = false;
          _status = 'Update error: $e';
        });
      }
    }
  }

  @override
  Widget build(BuildContext context) {
    final theme = widget.theme;
    final isAndroid = Platform.isAndroid;

    return PopScope(
      canPop: !_isDownloading,
      child: AlertDialog(
        backgroundColor: const Color(0xFF12141C),
        shape: RoundedRectangleBorder(borderRadius: BorderRadius.circular(16.0)),
        title: Row(
          children: [
            Container(
              padding: const EdgeInsets.all(8.0),
              decoration: BoxDecoration(
                color: theme.primary.withValues(alpha: 0.15),
                shape: BoxShape.circle,
              ),
              child: Icon(Icons.system_update_rounded, color: theme.primary, size: 24),
            ),
            const SizedBox(width: 12),
            const Expanded(
              child: Text(
                'Update Available',
                style: TextStyle(color: Colors.white, fontSize: 18, fontWeight: FontWeight.bold),
              ),
            ),
          ],
        ),
        content: Column(
          mainAxisSize: MainAxisSize.min,
          crossAxisAlignment: CrossAxisAlignment.start,
          children: [
            Container(
              padding: const EdgeInsets.symmetric(horizontal: 10.0, vertical: 6.0),
              decoration: BoxDecoration(
                color: Colors.white.withValues(alpha: 0.06),
                borderRadius: BorderRadius.circular(8.0),
              ),
              child: Row(
                mainAxisSize: MainAxisSize.min,
                children: [
                  const Icon(Icons.new_releases_outlined, color: Colors.cyanAccent, size: 16),
                  const SizedBox(width: 6),
                  Text(
                    'Version ${widget.release.tagName}',
                    style: const TextStyle(color: Colors.cyanAccent, fontSize: 12.5, fontWeight: FontWeight.bold),
                  ),
                ],
              ),
            ),
            const SizedBox(height: 12),
            const Text(
              'What\'s New:',
              style: TextStyle(color: Colors.white70, fontSize: 12, fontWeight: FontWeight.bold),
            ),
            const SizedBox(height: 4),
            Container(
              constraints: const BoxConstraints(maxHeight: 120),
              padding: const EdgeInsets.all(10.0),
              decoration: BoxDecoration(
                color: Colors.black.withValues(alpha: 0.3),
                borderRadius: BorderRadius.circular(8.0),
                border: Border.all(color: Colors.white.withValues(alpha: 0.05)),
              ),
              child: SingleChildScrollView(
                child: Text(
                  widget.release.releaseNotes.isEmpty
                      ? 'New enhancements, audio visualization improvements, and stability fixes.'
                      : widget.release.releaseNotes,
                  style: const TextStyle(color: Colors.white60, fontSize: 11.5, height: 1.4),
                ),
              ),
            ),
            const SizedBox(height: 14),

            if (_isDownloading) ...[
              ClipRRect(
                borderRadius: BorderRadius.circular(6),
                child: LinearProgressIndicator(
                  value: _progress > 0 ? _progress : null,
                  minHeight: 8,
                  backgroundColor: Colors.white.withValues(alpha: 0.1),
                  valueColor: AlwaysStoppedAnimation<Color>(theme.primary),
                ),
              ),
              const SizedBox(height: 8),
              Text(
                _status,
                style: TextStyle(color: theme.primary, fontSize: 11.5, fontWeight: FontWeight.w600),
              ),
            ] else ...[
              Text(
                isAndroid
                    ? 'Tap "Update Now" to download and install the latest update directly.'
                    : 'Tap "Update Now" to view the latest release details.',
                style: const TextStyle(color: Colors.white38, fontSize: 11),
              ),
            ],
          ],
        ),
        actions: [
          if (!_isDownloading)
            TextButton(
              onPressed: () => Navigator.pop(context),
              child: const Text('Later', style: TextStyle(color: Colors.white60)),
            ),
          if (!_isDownloading)
            FilledButton.icon(
              style: FilledButton.styleFrom(
                backgroundColor: theme.primary,
                foregroundColor: Colors.black,
              ),
              onPressed: _startUpdate,
              icon: const Icon(Icons.download_rounded, size: 18),
              label: const Text('Update Now', style: TextStyle(fontWeight: FontWeight.bold)),
            ),
        ],
      ),
    );
  }
}
