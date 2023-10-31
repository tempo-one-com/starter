# Installation de Rust
## Linux / Mac
```sudo curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sudo sh```
ou
## Windows
```https://static.rust-lang.org/rustup/dist/x86_64-pc-windows-msvc/rustup-init.exe```

# Build (debug)
```cargo build```

# Build (optimisé pour prod)
```cargo build --release```

# Copier le fichier binaire à la racine du projet
## Linux / Mac
```ln -s target/debug/starter .```  
ou  
## Windows
```copy target\debug\starter.exe .```

# Exécution
Le programme va exécuter en parallèle 3 processus de long_script avec les arguments suivant (1, -1 et 3).
3 fichiers correspondant au résultat de chacune des exécutions seront écrits dans ce répertoire.  

## Linux / Mac
```./starter```  
ou  
## Windows
```.\starter.exe``` 