// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'main.dart';

// **************************************************************************
// StoreGenerator
// **************************************************************************

// ignore_for_file: non_constant_identifier_names, unnecessary_lambdas, prefer_expression_function_bodies, lines_longer_than_80_chars, avoid_as, avoid_annotating_with_dynamic

mixin _$MainStore on _MainStore, Store {
  final _$fiatAtom = Atom(name: '_MainStore.fiat');

  @override
  Fiat get fiat {
    _$fiatAtom.context.enforceReadPolicy(_$fiatAtom);
    _$fiatAtom.reportObserved();
    return super.fiat;
  }

  @override
  set fiat(Fiat value) {
    _$fiatAtom.context.conditionallyRunInAction(() {
      super.fiat = value;
      _$fiatAtom.reportChanged();
    }, _$fiatAtom, name: '${_$fiatAtom.name}_set');
  }

  final _$initPrepAsyncAction = AsyncAction('initPrep');

  @override
  Future<void> initPrep() {
    return _$initPrepAsyncAction.run(() => super.initPrep());
  }
}
