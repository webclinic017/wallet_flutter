import 'package:flutter/material.dart';
import 'package:flutter/services.dart';
import 'package:flutter_mobx/flutter_mobx.dart';
import 'package:provider/provider.dart';
import 'package:pull_to_refresh/pull_to_refresh.dart';
import 'package:wallet_flutter/utils/app_localization.dart';
import 'package:wallet_flutter/utils/constants.dart';
import 'package:wallet_flutter/gen/cargo/protos/coin.pb.dart';
import 'package:wallet_flutter/utils/fn.dart';
import 'package:wallet_flutter/stores/main.dart';
import 'package:http/http.dart' as http;

const double tsize = 20;
const ts1 = TextStyle(fontSize: tsize);
const ts2 = TextStyle(fontSize: tsize, fontWeight: FontWeight.bold);


class Wallet extends StatefulWidget {
  const Wallet({
    Key key,
  }) : super(key: key);

  @override
  _WalletState createState() => _WalletState();
}

RefreshController _refreshController = RefreshController(initialRefresh: false);
class _WalletState extends State<Wallet> {

  final receivingAddress = TextEditingController();
  final amount = TextEditingController();

  @override
  Widget build(context){
    final walletStore = Provider.of<MainStore>(context).walletStore;
    final mainStore = Provider.of<MainStore>(context);

    return SmartRefresher(
      enablePullDown: true,
      enablePullUp: true,
      controller: _refreshController,
      onRefresh: () async{
        await walletStore.refreshBalances();
        _refreshController.refreshCompleted();
      },
      header: ClassicHeader(),
      footer: CustomFooter(
        builder: (BuildContext context, LoadStatus mode) {
            return Container(
              height: 55.0,
              child: Center(),
            );
        },
      ), 
      child: Observer(
        builder: (_) {
          Coins a = mainStore.coinListFromBase;
          Coin x = mainStore.coinFromRel;
          
          var b = walletStore.getBalance(rel: x.rel, base: a.base);
          return Padding(
          padding: EdgeInsets.symmetric(horizontal: 10, vertical: 10),
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
                          Text("${x.rel} balance".toUpperCase(), style: ts1),
                          Text(b.balance.toStringAsFixed(CRYPTO_PRECISION), style: ts2),
                        ],
                      ),
                      Column(
                        crossAxisAlignment: CrossAxisAlignment.start,
                        children: <Widget>[
                          Text("USD value".toUpperCase(), style: ts1),
                          Text("${walletStore.fiat.symbol}${b.fiat.toStringAsFixed(FIAT_PRECISION)}", style: ts2),
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
                      labelText: 'Your ${x.rel} Address',
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
                  ScanTextField(controller: receivingAddress, onScan: (text) => this.receivingAddress.text = text),
                  TextField(
                    keyboardType: TextInputType.number,
                    controller: amount,
                    decoration: InputDecoration(
                      border: OutlineInputBorder(),
                      labelText: '${x.rel.toUpperCase()} Amount',
                      suffixIcon: FlatButton(
                        child: Text("MAX"), 
                        onPressed: () {
                          amount.text = b.balance.toString();
                        },
                      ),
                    ),
                  ),
                  SizedBox(
                    width: double.infinity,
                    child: RaisedButton(
                      onPressed: () async{
                        
                        Outputs os = Outputs();
                        Output o = Output()
                        ..address = receivingAddress.text
                        ..value = textToDouble(amount.text)
                        ..memo = "";
                        o.address = receivingAddress.text;

                        os.list.add(o);
                        

                        var input = GenSendTxInput()
                        ..config = getConfig(x.rel, a.base)
                        ..privateKey = x.privateKey
                        ..publicKey = x.publicKey
                        ..outputs = os;

                        //mainStore.rust.invokeRustMethod('gen_send_transaction', input.writeToBuffer(),(x){
                         // var t = Tx.fromBuffer(x);
                          //sendTransaction(x.rel, a.base, t.txHex);
                        //});
                      },

                      //padding: EdgeInsets.all(15),
                      //shape: new RoundedRectangleBorder(borderRadius: new BorderRadius.circular(8.0)),
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
      ),
    );
  }
}

Future<String> sendTransaction(String rel, String base, String rawTx) async {
  var response = await http.post("${explorerApi}/post_tx", body: {'rel': rel, 'base': base, 'rawTx': rawTx});
  print('Response body: ${response.body}');
}

class ScanTextField extends StatelessWidget {
  ScanTextField({this.controller, this.onScan});

  final TextEditingController controller;
  final Function onScan;

  @override
  Widget build(BuildContext context) {
    return TextField(
      controller: controller,
      decoration: InputDecoration(
        border: OutlineInputBorder(),
        labelText: 'Recieving Address',
        prefixIcon: IconButton(
          icon: Icon(Icons.center_focus_weak),
          onPressed: (){
            scan(onScan);
          }
        ),
        suffixIcon: IconButton(
          icon: Icon(Icons.content_paste),
          onPressed: () {
            writePaste(this.controller);
          }
        ),
      ),
    );
  }
}