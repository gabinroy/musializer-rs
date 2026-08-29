import 'dart:convert';
import 'dart:io';
import 'package:flutter/foundation.dart';
import 'package:http/http.dart' as http;
import 'package:open_filex/open_filex.dart';
import 'package:package_info_plus/package_info_plus.dart';
import 'package:url_launcher/url_launcher.dart';

class AppReleaseInfo {
  final String tagName;
  final String version;
  final String releaseNotes;
  final String? apkDownloadUrl;
  final String htmlUrl;
  final String publishedAt;

  AppReleaseInfo({
    required this.tagName,
    required this.version,
    required this.releaseNotes,
    required this.apkDownloadUrl,
    required this.htmlUrl,
    required this.publishedAt,
  });
}

class UpdateService {
  static const String repoOwner = 'gabinroy';
  static const String repoName = 'musializer-rs';

  /// Checks GitHub API for the latest published release.
  /// Returns [AppReleaseInfo] if a newer version exists than currently installed.
  static Future<AppReleaseInfo?> checkForUpdates() async {
    try {
      final packageInfo = await PackageInfo.fromPlatform();
      final currentVerStr = packageInfo.version; // e.g. "0.1.1"

      final url = Uri.parse('https://api.github.com/repos/$repoOwner/$repoName/releases/latest');
      final response = await http.get(url, headers: {
        'Accept': 'application/vnd.github.v3+json',
        'User-Agent': 'Musializer-Mobile-App',
      }).timeout(const Duration(seconds: 8));

      if (response.statusCode != 200) {
        debugPrint('GitHub Release API returned status ${response.statusCode}');
        return null;
      }

      final data = jsonDecode(response.body) as Map<String, dynamic>;
      final tagName = (data['tag_name'] as String? ?? '').trim();
      final htmlUrl = data['html_url'] as String? ?? 'https://github.com/$repoOwner/$repoName/releases';
      final body = data['body'] as String? ?? 'Bug fixes and performance improvements.';
      final publishedAt = data['published_at'] as String? ?? '';

      // Clean version tag: "v0.1.2" -> "0.1.2"
      final cleanTag = tagName.startsWith('v') || tagName.startsWith('V')
          ? tagName.substring(1)
          : tagName;

      if (_isVersionNewer(cleanTag, currentVerStr)) {
        // Find APK asset for Android
        String? apkUrl;
        final assets = data['assets'] as List<dynamic>? ?? [];
        for (final asset in assets) {
          final name = (asset['name'] as String? ?? '').toLowerCase();
          final downloadUrl = asset['browser_download_url'] as String?;
          if (name.endsWith('.apk') && downloadUrl != null) {
            apkUrl = downloadUrl;
            break;
          }
        }

        return AppReleaseInfo(
          tagName: tagName,
          version: cleanTag,
          releaseNotes: body,
          apkDownloadUrl: apkUrl,
          htmlUrl: htmlUrl,
          publishedAt: publishedAt,
        );
      }
    } catch (e) {
      debugPrint('Error checking for updates: $e');
    }
    return null;
  }

  /// Downloads the APK and triggers the native Android package installer.
  /// On iOS, opens the GitHub Releases / App Store page.
  static Future<void> downloadAndInstallUpdate({
    required AppReleaseInfo release,
    required void Function(double progress, String status) onProgress,
  }) async {
    if (Platform.isAndroid && release.apkDownloadUrl != null) {
      onProgress(0.05, 'Starting download...');

      final client = http.Client();
      final request = http.Request('GET', Uri.parse(release.apkDownloadUrl!));
      request.headers['User-Agent'] = 'Musializer-Mobile-App';

      final response = await client.send(request);
      final totalBytes = response.contentLength ?? 0;
      int downloadedBytes = 0;

      final tempDir = Directory.systemTemp;
      final apkFile = File('${tempDir.path}/musializer_${release.version}.apk');
      final sink = apkFile.openWrite();

      await response.stream.listen((chunk) {
        sink.add(chunk);
        downloadedBytes += chunk.length;
        if (totalBytes > 0) {
          final progress = (downloadedBytes / totalBytes).clamp(0.0, 1.0);
          onProgress(
            progress,
            'Downloading: ${(downloadedBytes / (1024 * 1024)).toStringAsFixed(1)} MB / ${(totalBytes / (1024 * 1024)).toStringAsFixed(1)} MB',
          );
        }
      }).asFuture();

      await sink.flush();
      await sink.close();

      onProgress(1.0, 'Launching package installer...');

      // Trigger Android native package installer prompt
      final openResult = await OpenFilex.open(
        apkFile.path,
        type: 'application/vnd.android.package-archive',
      );

      debugPrint('APK install open result: ${openResult.type} - ${openResult.message}');
    } else {
      // iOS / Web / Fallback: Launch release browser page
      final uri = Uri.parse(release.htmlUrl);
      if (await canLaunchUrl(uri)) {
        await launchUrl(uri, mode: LaunchMode.externalApplication);
      }
    }
  }

  /// Compares semantic versions: returns true if [remote] > [current]
  static bool _isVersionNewer(String remote, String current) {
    try {
      final rParts = remote.split('.').map((e) => int.tryParse(RegExp(r'\d+').stringMatch(e) ?? '0') ?? 0).toList();
      final cParts = current.split('.').map((e) => int.tryParse(RegExp(r'\d+').stringMatch(e) ?? '0') ?? 0).toList();

      while (rParts.length < 3) {
        rParts.add(0);
      }
      while (cParts.length < 3) {
        cParts.add(0);
      }

      for (int i = 0; i < 3; i++) {
        if (rParts[i] > cParts[i]) return true;
        if (rParts[i] < cParts[i]) return false;
      }
    } catch (_) {}
    return false;
  }
}
