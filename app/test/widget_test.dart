import 'package:flutter_test/flutter_test.dart';
import 'package:lingo_app/main.dart';

void main() {
  testWidgets('length selection opens a game screen', (tester) async {
    await tester.pumpWidget(const LingoApp());
    await tester.tap(find.text('4 harf'));
    await tester.pumpAndSettle();
    expect(find.text('4 harfli kelime'), findsOneWidget);
    expect(find.text('Hak: 5 / 5'), findsOneWidget);
  });
}
