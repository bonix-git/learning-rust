# learning-rust
Aprendendo Rust

# Livro de referência

 - The Rust Programming Language, 2nd Edition, Steve Klabnik & Carol Nichols


 # Extensões para VS Code

https://blog.logrocket.com/top-10-vs-code-extensions-2021/

Adicionar o comando "code" ao PATH:
'''
export PATH="$PATH:/Applications/Visual Studio Code.app/Contents/Resources/app/bin"
source ~/.zshrc
'''

Exportar:
'''
code --list-extensions > vscode-extensions.txt
'''

Importar: 
'''
cat vscode-extensions.txt | xargs -n 1 code --install-extension
'''

