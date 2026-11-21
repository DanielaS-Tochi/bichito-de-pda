import { useClient } from "@solana/react";
import {
  useConnect,
  useConnectedWallet,
  useDisconnect,
  useWalletStatus,
  useWallets,
} from "@solana/kit-plugin-wallet/react";
import type { AppClient } from "./client";
import "./App.css";

function App() {
  const client = useClient<AppClient>();
  const status = useWalletStatus(client);
  const wallets = useWallets(client);
  const connected = useConnectedWallet(client);
  const connect = useConnect(client);
  const disconnect = useDisconnect(client);

  const walletAddress = connected?.account.address;
  const shortAddress = walletAddress
    ? `${walletAddress.slice(0, 4)}…${walletAddress.slice(-4)}`
    : null;

  return (
    <main className="app-shell">
      <header className="topbar">
        <a className="brand" href="/" aria-label="Bichito de PDA, inicio">
          <span className="brand-mark" aria-hidden="true">✿</span>
          <span>Bichito <span className="brand-light">de PDA</span></span>
        </a>

        <span className="network-badge">
          <span className="network-dot" />
          Solana · Devnet
        </span>
      </header>

      <section className="welcome">
        <div className="welcome-copy">
          <p className="eyebrow">Una aventura pequeñita, en la blockchain</p>
          <h1>Una mascota que<br />vive en Solana.</h1>
          <p className="welcome-text">
            Elegí su nombre, cuidala y descubrí cómo cada cambio queda guardado
            en una cuenta PDA.
          </p>
        </div>

        <div className="mascot-card" aria-label="Ilustración de una mascota">
          <div className="sparkle sparkle-one">✦</div>
          <div className="sparkle sparkle-two">✧</div>
          <div className="mascot">🐣</div>
          <div className="mascot-caption">
            <span className="caption-dot" />
            Tu próxima compañera te espera
          </div>
        </div>
      </section>

      <section className="connect-card" aria-labelledby="connect-title">
        <div className="connect-icon" aria-hidden="true">⌑</div>
        <div className="connect-content">
          <p className="eyebrow">Primer paso</p>
          <h2 id="connect-title">
            {connected ? "¡Ya estamos conectadas!" : "Conectá tu wallet"}
          </h2>

          {connected ? (
            <>
              <p className="connect-description">
                Tu wallet está lista para interactuar con el programa en Devnet.
              </p>
              <div className="wallet-address" title={walletAddress ?? ""}>
                <span className="network-dot" />
                {shortAddress}
              </div>
              {!connected.signer && (
                <p className="wallet-note" role="status">
                  Esta conexión es de solo lectura. Para cuidar a tu bichito,
                  conectá una wallet que permita firmar transacciones.
                </p>
              )}
              <button
                className="button button-secondary"
                type="button"
                onClick={() => disconnect.dispatch()}
                disabled={disconnect.isRunning}
              >
                {disconnect.isRunning ? "Desconectando…" : "Desconectar"}
              </button>
            </>
          ) : (
            <>
              <p className="connect-description">
                La wallet identifica quién es la dueña de la mascota y autoriza
                sus cuidados.
              </p>
              {status === "pending" ? (
                <p className="wallet-note" role="status">
                  Buscando wallets…
                </p>
              ) : wallets.length > 0 ? (
                <div className="wallet-options">
                  {wallets.map((wallet) => (
                    <button
                      className="button button-primary"
                      type="button"
                      key={wallet.name}
                      onClick={() => connect.dispatch(wallet)}
                      disabled={connect.isRunning}
                    >
                      {connect.isRunning
                        ? "Conectando…"
                        : `Conectar ${wallet.name}`}
                    </button>
                  ))}
                </div>
              ) : (
                <p className="wallet-note" role="status">
                  No encontramos una wallet compatible en este navegador.
                  Abrí la app en un navegador donde tengas una wallet de Solana.
                </p>
              )}
              {connect.error && (
                <p className="wallet-error" role="alert">
                  No se pudo conectar: {String(connect.error)}
                </p>
              )}
            </>
          )}
        </div>
      </section>

      <footer className="footer">
        <span>Hecho para aprender, paso a paso.</span>
        <span>Tu mascota. Tu wallet. Tu PDA.</span>
      </footer>
    </main>
  );
}

export default App;
