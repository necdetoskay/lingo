import 'package:flutter/material.dart';

void main() => runApp(const LingoApp());

class LingoApp extends StatelessWidget {
  const LingoApp({super.key});
  @override
  Widget build(BuildContext context) => MaterialApp(
        title: 'Lingo',
        theme: ThemeData(colorScheme: ColorScheme.fromSeed(seedColor: Colors.teal), useMaterial3: true),
        home: const HomeScreen(),
      );
}

class HomeScreen extends StatelessWidget {
  const HomeScreen({super.key});
  @override
  Widget build(BuildContext context) => Scaffold(
        appBar: AppBar(title: const Text('Lingo')),
        body: Center(child: Padding(
          padding: const EdgeInsets.all(24),
          child: Column(mainAxisSize: MainAxisSize.min, children: [
            const Text('Türkçe kelime oyunu', style: TextStyle(fontSize: 28, fontWeight: FontWeight.bold)),
            const SizedBox(height: 12),
            const Text('4, 5 veya 6 harfli bir kelime seçerek başla. Her tahmin için 20 saniyen var.', textAlign: TextAlign.center),
            const SizedBox(height: 24),
            Wrap(spacing: 12, children: [4, 5, 6].map((length) => FilledButton(onPressed: () => ScaffoldMessenger.of(context).showSnackBar(SnackBar(content: Text('$length harfli oyun Rust çekirdeği bağlandığında başlayacak.'))), child: Text('$length harf'))).toList()),
          ]),
        )),
      );
}
