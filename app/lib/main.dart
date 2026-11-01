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
            Wrap(spacing: 12, children: [4, 5, 6].map((length) => FilledButton(onPressed: () => Navigator.of(context).push(MaterialPageRoute<void>(builder: (_) => GameScreen(length: length))), child: Text('$length harf'))).toList()),
          ]),
        )),
      );
}

class GameScreen extends StatefulWidget {
  const GameScreen({required this.length, super.key});
  final int length;
  @override
  State<GameScreen> createState() => _GameScreenState();
}

class _GameScreenState extends State<GameScreen> {
  final _controller = TextEditingController();
  int _remainingAttempts = 5;
  @override
  void dispose() { _controller.dispose(); super.dispose(); }
  @override
  Widget build(BuildContext context) => Scaffold(
    appBar: AppBar(title: Text('${widget.length} harfli kelime')),
    body: Padding(
      padding: const EdgeInsets.all(24),
      child: Column(children: [
        Row(mainAxisAlignment: MainAxisAlignment.spaceBetween, children: [Text('Hak: $_remainingAttempts / 5'), const Text('20 sn')]),
        const SizedBox(height: 24),
        Row(mainAxisAlignment: MainAxisAlignment.center, children: List.generate(widget.length, (index) => Container(
          width: 42, height: 48, margin: const EdgeInsets.all(3), alignment: Alignment.center,
          decoration: BoxDecoration(border: Border.all(color: Theme.of(context).colorScheme.outline), borderRadius: BorderRadius.circular(6)),
          child: Text(index == 0 ? '?' : '', style: const TextStyle(fontSize: 22)),
        ))),
        const Spacer(),
        TextField(controller: _controller, maxLength: widget.length, textCapitalization: TextCapitalization.characters, decoration: const InputDecoration(labelText: 'Tahminin')),
        FilledButton(onPressed: _remainingAttempts == 0 ? null : () { setState(() => _remainingAttempts--); _controller.clear(); }, child: const Text('Tahmin et')),
      ]),
    ),
  );
}
