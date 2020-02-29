import 'package:flutter/material.dart';
import 'package:wallet_flutter/models/balance.dart';
import 'package:wallet_flutter/models/transaction.dart' as T;
import 'package:wallet_flutter/stores/balance.dart';
import 'package:wallet_flutter/utils/constants.dart';
import 'package:wallet_flutter/utils/fn.dart';

import 'tx_labels.dart';

class TxIO extends StatelessWidget {
  final T.Tx t;
  final String rel;
  final Fiat fiat;
  final Balance b;
  TxIO({this.t, this.rel, this.fiat, this.b});
  @override
  Widget build(BuildContext context) {
    var v = valueToPrecision(t.value, rel);
    return Column(
      crossAxisAlignment: CrossAxisAlignment.start,
      children: <Widget>[
        TxLabels(label: "Address", value: t.address),
        TxLabels(
            label: "Value",
            value:
                "${valueToPretty(v, CRYPTO_PRECISION)} (${fiat.symbol}${valueToPretty(v * b.price, FIAT_PRECISION)})"),
        Divider(),
      ],
    );
  }
}
