SETUP
Instale o NDK
Abra o arquivo .bashrc com comando gedit ~/.bashrc e adicione no final do arquivo:

  export ANDROID_HOME=$HOME/Android/Sdk
  export PATH=$PATH:$ANDROID_HOME/tools 
  export PATH=$PATH:$ANDROID_HOME/platform-tools
  export ANDROID_NDK_HOME="$ANDROID_HOME/ndk/25.2.9519653" (adicione o caminho do NDK corretamente, pode ser diferente de 25.2.9519653)

Com as variáveis de ambiente configuradas, execute os comandos:
mkdir ~/.NDK
$ANDROID_NDK_HOME/build/tools/make_standalone_toolchain.py --api 26 --arch arm64 --install-dir ~/.NDK/arm64;
$ANDROID_NDK_HOME/build/tools/make_standalone_toolchain.py --api 26 --arch arm --install-dir ~/.NDK/arm;
$ANDROID_NDK_HOME/build/tools/make_standalone_toolchain.py --api 26 --arch x86 --install-dir ~/.NDK/x86;

Aplique as alterações com o comando: source ~/.bashrc
