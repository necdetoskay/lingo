import 'package:flutter/material.dart';
import 'package:lingo_app/src/rust/frb_generated.dart';

Future<void> main() async {
  WidgetsFlutterBinding.ensureInitialized();
  await RustLib.init();
  runApp(const LingoApp());
}

class LingoApp extends StatelessWidget {
  const LingoApp({super.key});
  @override
  Widget build(BuildContext context) => MaterialApp(home: const HomeScreen());
}

class HomeScreen extends StatelessWidget {
  const HomeScreen({super.key});
  @override
  Widget build(BuildContext context) => Scaffold(
    appBar: AppBar(title: const Text('Lingo')),
    body: Center(child: FilledButton(onPressed: () => Navigator.of(context).push(MaterialPageRoute<void>(builder: (_) => const GameScreen(length: 4))), child: const Text('4 harf'))),
  );
}

class GameScreen extends StatelessWidget {
  const GameScreen({required this.length, super.key});
  final int length;
  @override
  Widget build(BuildContext context) => Scaffold(appBar: AppBar(title: Text('$length harfli kelime')), body: const Center(child: Text('Hak: 5 / 5')));
}
