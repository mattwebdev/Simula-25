import * as vscode from 'vscode';
import {
    LanguageClient,
    LanguageClientOptions,
    ServerOptions,
    TransportKind
} from 'vscode-languageclient/node';

let client: LanguageClient;

export function activate(context: vscode.ExtensionContext) {
    // The server is implemented in Rust
    const serverModule = context.asAbsolutePath('server/simula-lsp');
    
    // If the extension is launched in debug mode then the debug server options are used
    const serverOptions: ServerOptions = {
        run: { command: serverModule, transport: TransportKind.stdio },
        debug: {
            command: serverModule,
            transport: TransportKind.stdio,
            args: ['--debug']
        }
    };

    // Options to control the language client
    const clientOptions: LanguageClientOptions = {
        // Register the server for Simula documents
        documentSelector: [{ scheme: 'file', language: 'simula' }],
        synchronize: {
            // Notify the server about file changes to '.simula' files contained in the workspace
            fileEvents: vscode.workspace.createFileSystemWatcher('**/*.simula')
        }
    };

    // Create the language client and start the client
    client = new LanguageClient(
        'simula-25',
        'Simula 25 Language Server',
        serverOptions,
        clientOptions
    );

    // Start the client. This will also launch the server
    client.start();

    // Register the simulation preview command
    let disposable = vscode.commands.registerCommand('simula-25.showSimulationPreview', () => {
        // Create and show panel
        const panel = vscode.window.createWebviewPanel(
            'simulaSimulation',
            'Simulation Preview',
            vscode.ViewColumn.Two,
            {
                enableScripts: true
            }
        );

        // TODO: Implement simulation preview
        panel.webview.html = getWebviewContent();
    });

    context.subscriptions.push(disposable);
}

export function deactivate(): Thenable<void> | undefined {
    if (!client) {
        return undefined;
    }
    return client.stop();
}

function getWebviewContent() {
    return `<!DOCTYPE html>
    <html lang="en">
    <head>
        <meta charset="UTF-8">
        <meta name="viewport" content="width=device-width, initial-scale=1.0">
        <title>Simula 25 Simulation Preview</title>
    </head>
    <body>
        <h1>Simulation Preview</h1>
        <p>Coming soon...</p>
    </body>
    </html>`;
} 