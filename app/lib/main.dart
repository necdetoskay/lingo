import 'dart:async';

import 'package:flutter/material.dart';
import 'package:lingo_app/src/rust/api/game.dart';
import 'package:lingo_app/src/rust/frb_generated.dart';

Future<void> main() async {
  WidgetsFlutterBinding.ensureInitialized();
  await RustLib.init();
  runApp(const LingoApp());
}

class LingoApp extends StatelessWidget {
  const LingoApp({super.key});
  @override
  Widget build(BuildContext context) => MaterialApp(
        title: 'Lingo',
        theme: ThemeData(colorSchemeSeed: Colors.teal, useMaterial3: true),
        home: const HomeScreen(),
      );
}

class HomeScreen extends StatelessWidget {
  const HomeScreen({super.key});
  @override
  Widget build(BuildContext context) => Scaffold(
        appBar: AppBar(title: const Text('Lingo')),
        body: Center(
          child: Column(
            mainAxisSize: MainAxisSize.min,
            children: [
              const Text('Kelime uzunluğunu seç'),
              const SizedBox(height: 16),
              for (final length in [4, 5, 6])
                Padding(
                  padding: const EdgeInsets.symmetric(vertical: 4),
                  child: FilledButton(
                    onPressed: () => Navigator.of(context).push(
                        MaterialPageRoute<void>(
                            builder: (_) => GameScreen(length: length))),
                    child: Text('$length harf'),
                  ),
                ),
            ],
          ),
        ),
      );
}

class GameScreen extends StatefulWidget {
  const GameScreen({required this.length, super.key});
  final int length;

  @override
  State<GameScreen> createState() => _GameScreenState();
}

class _GameScreenState extends State<GameScreen> {
  final _guess = TextEditingController();
  RoundSnapshot? _round;
  String _message = '';
  List<int> _feedback = const [];
  Timer? _timer;

  int get _now => DateTime.now().millisecondsSinceEpoch;

  @override
  void initState() {
    super.initState();
    _start();
    _timer = Timer.periodic(const Duration(seconds: 1), (_) => _sync());
  }

  @override
  void dispose() {
    _timer?.cancel();
    _guess.dispose();
    super.dispose();
  }

  Future<void> _start() async {
    try {
      final round = await startRound(wordLength: widget.length, nowMs: _now);
      if (mounted) {
        setState(() {
          _round = round;
          _feedback = const [];
          _message = '';
        });
      }
    } catch (_) {
      if (mounted) setState(() => _message = 'Oyun motoru başlatılamadı.');
    }
  }

  Future<void> _sync() async {
    if (_round == null || _round!.status != 'in_progress') return;
    try {
      final round = await reconcileRound(nowMs: _now);
      if (mounted) {
        setState(() => _round = round);
      }
    } catch (_) {}
  }

  Future<void> _submit() async {
    final revealed = _round?.revealedLetters;
    final firstLetter =
        revealed == null || revealed.isEmpty ? null : revealed.first;
    if (firstLetter == null) {
      setState(() => _message = 'Tur hazırlanıyor.');
      return;
    }
    if (_guess.text.runes.length != widget.length - 1) {
      setState(() => _message = '${widget.length - 1} harf daha girin.');
      return;
    }
    try {
      final result = await submitGuess(
        guess: '$firstLetter${_guess.text}',
        nowMs: _now,
      );
      if (!mounted) return;
      setState(() {
        _round = result.snapshot;
        _feedback = result.feedback;
        _message = result.result == 'invalid_word'
            ? 'Bu kelime sözlükte yok veya ilk harfle başlamıyor.'
            : result.score == null
                ? ''
                : 'Tebrikler! ${result.score} puan.';
      });
      _guess.clear();
    } catch (_) {
      if (mounted) setState(() => _message = 'Tahmin işlenemedi.');
    }
  }

  @override
  Widget build(BuildContext context) {
    final round = _round;
    final seconds = round == null
        ? 20
        : ((round.deadlineMs - _now) / 1000).ceil().clamp(0, 20);
    final letters =
        round?.revealedLetters ?? List<String?>.filled(widget.length, null);
    final isOver = round != null && round.status != 'in_progress';
    return Scaffold(
      appBar: AppBar(title: Text('${widget.length} harfli kelime')),
      body: Padding(
        padding: const EdgeInsets.all(20),
        child: Column(children: [
          Text('Hak: ${round?.attemptsRemaining ?? 5} / 5',
              style: Theme.of(context).textTheme.titleLarge),
          Text('Süre: $seconds sn'),
          const SizedBox(height: 24),
          Row(
              mainAxisAlignment: MainAxisAlignment.center,
              children: List.generate(letters.length, (index) {
                final color = index < _feedback.length
                    ? (_feedback[index] == 2
                        ? Colors.green
                        : _feedback[index] == 1
                            ? Colors.amber
                            : Colors.grey)
                    : Colors.blueGrey;
                return Container(
                    width: 44,
                    height: 44,
                    margin: const EdgeInsets.all(3),
                    alignment: Alignment.center,
                    color: color,
                    child: Text(letters[index] ?? '',
                        style: const TextStyle(
                            color: Colors.white, fontSize: 20)));
              })),
          const SizedBox(height: 18),
          Row(children: [
            Container(
                width: 44,
                height: 48,
                alignment: Alignment.center,
                color: Colors.blueGrey,
                child: Text(letters.first ?? '',
                    style: const TextStyle(color: Colors.white, fontSize: 20))),
            const SizedBox(width: 8),
            Expanded(
                child: TextField(
                    controller: _guess,
                    maxLength: widget.length - 1,
                    textCapitalization: TextCapitalization.characters,
                    textAlign: TextAlign.center,
                    decoration: InputDecoration(
                        labelText: 'Kalan ${widget.length - 1} harf',
                        helperText: _message),
                    onSubmitted: (_) => _submit())),
          ]),
          FilledButton(
              onPressed: isOver ? _start : _submit,
              child: Text(isOver ? 'Yeni tur' : 'Tahmin et')),
        ]),
      ),
    );
  }
}
