import 'dart:convert';

import 'package:easy_localization/easy_localization.dart';
import 'package:flutter/material.dart';
import 'package:flutter/services.dart';
import 'package:flutter_mobx/flutter_mobx.dart';
import 'package:provider/provider.dart';
import 'package:wallet_flutter/models/tx_output.dart';
import 'package:wallet_flutter/screens/scan.dart';
import 'package:wallet_flutter/stores/main.dart';

const platform = const MethodChannel('flutter.dev/rust');

const double tsize = 20;
const ts1 = TextStyle(fontSize: tsize);
const ts2 = TextStyle(fontSize: tsize, fontWeight: FontWeight.bold);

class Wallet extends StatelessWidget {
  final receivingAddress = TextEditingController();
  final amount = TextEditingController();

  @override
  Widget build(context){
    final walletStore = Provider.of<MainStore>(context).walletStore;
    final fabStore = Provider.of<MainStore>(context).fabStore;


    return Observer(
      builder: (_) {
        var selected = fabStore.selected;        
        var selectedChild = fabStore.selectedChild;
        var x = walletStore.wList[0].coinbaseList[selected].coins[selectedChild];
        return Padding(
        padding: EdgeInsets.symmetric(horizontal: 30, vertical: 10),
          child: SingleChildScrollView(
            child: Wrap(
              runSpacing: 30,
              children: <Widget>[
                Row(
                  mainAxisAlignment: MainAxisAlignment.spaceBetween,
                  children: <Widget>[
                    Column(
                      crossAxisAlignment: CrossAxisAlignment.start,
                      children: <Widget>[
                        Text("${x.ticker} balance".toUpperCase(), style: ts1),
                        Text("0", style: ts2),
                      ],
                    ),
                    Column(
                      crossAxisAlignment: CrossAxisAlignment.start,
                      children: <Widget>[
                        Text("USD value".toUpperCase(), style: ts1),
                        Text("\$0", style: ts2),
                      ],
                    ),
                  ],
                ),
                TextField(
                  readOnly: true,
                  controller: TextEditingController(text: x.address),
                  style: TextStyle(color: Colors.grey),
                  decoration: InputDecoration(
                    border: OutlineInputBorder(),
                    labelText: 'Your ${x.ticker} Address',
                    suffixIcon: IconButton(
                      icon: Icon(Icons.content_copy),
                      onPressed: () {
                        Clipboard.setData(ClipboardData(text: x.address));
                      }
                    ),
                  ),
                ),
                Text(
                  AppLocalizations.of(context).tr('send_tx').toUpperCase(),
                  style:  TextStyle(
                    color: Color(0xffbec0c4),
                    fontWeight: FontWeight.bold,
                    fontSize: 14,
                  )
                ),
                ScanScreen(controller: receivingAddress, onScan: (text) => this.receivingAddress.text = text),
                TextField(
                  keyboardType: TextInputType.number,
                  controller: amount,
                  decoration: InputDecoration(
                    border: OutlineInputBorder(),
                    labelText: '${x.ticker} Amount',
                    suffixIcon: IconButton(
                      icon: Icon(Icons.content_paste),
                      onPressed: () {
                        Clipboard.getData("text/plain");
                      }
                    ),
                  ),
                ),
                SizedBox(
                  width: double.infinity,
                  child: RaisedButton(
                    onPressed: () async{
                      
                      String txOutputs = jsonEncode([TxOutput(address: receivingAddress.text, value: int.parse(amount.text))]);
                      String tx_signed_hex = await platform.invokeMethod('gen_send_transaction',{"ticker": x.ticker, "private_key": x.private_key, "public_key": x.public_key, "tx_outputs": txOutputs});
                      //send tx_signed_hex
                    },
                    padding: EdgeInsets.all(15),
                    shape: new RoundedRectangleBorder(borderRadius: new BorderRadius.circular(8.0)),
                    child: Text(
                      'Send',
                      style: TextStyle(fontSize: 20)
                    ),
                  ),
                ),
              ],
            ),
          ),
        );
      }
    );
  }
}

Future<String> send_transaction() async {

}