# dejavu

Tap-to-meet mobile app for remembering your new friends, built on Solana Mobile
Stack

Installation instrcutions

0. React Natice environment setup. Follow the instructions from [React
   Native](https://reactnative.dev/docs/environment-setup). Toggle option Target
   OS to Android.

1. Clone this repository.

```shell
 git clone git@github.com:aroralanuk/dejavu.git
```

2. Clone mobile-wallet-adapter repo in the same directory as dejavu

   ```shell
   git clone https://github.com/solana-mobile/mobile-wallet-adapter
   cd js
   yarn
   yarn build
   ```

3. Build the app

```shell
    cd dejavu
    yarn
    android link
    yarn android
```
