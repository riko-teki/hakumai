# hakumai
base64 encoder / decoder

# エンコード手順
1. 入力文字列をバイト列に変換
1. バイト列のbit数を確認
24bitの倍数の場合、エンコード結果はパディング無し、4の倍数個の文字になる。
総bit数 / 24bit のあまりが8bitの場合、エンコード結果はパディングが2文字になる。
総bit数 / 24bit のあまりが16bitの場合、エンコード結果はパディングが１文字になる。

