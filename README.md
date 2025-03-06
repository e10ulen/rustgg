# これなに

Golang製のツール  
gg のrust版です。  
習作として最初はコマンドラインツールとして、  
のちにgit実装を目指しています。  

# 現状
エラー処理として ```git init``` をかけようか悩んでます。    
自動実行でpushまで行えるようになりました。  
``` rustgg ``` でadd~pushまで行う。  
本当にこれでいいのか？  
Argsを取って、 ``` rustgg g ``` で自動pushまで行える方がいいのでは？  
## 実行方法
``` rustgg add``` -> ``` git add . ```  
``` rustgg commit {MESSAGE}``` -> ``` git commit -m "{TEXTMESSAGE}```   
``` rustgg push``` -> ``` git push -u ```  
``` rustgg all``` -> ``` git add . && git commit -m "[fix]{UTC}" && git push -u ```

# 目標
## commitに関して
メッセージ部分を引数何もなしだと日付時刻あたりを
引数fixなら修正したファイル名がわかるように
## remoteに関して
```git remote origin main git@example.com:user/repo``` を実装できるように、
対話形式プロンプトか何かを実装していきたい。
