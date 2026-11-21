import { useEffect, useState } from "react";
import { useClient } from "@solana/react";
import { useConnectedWallet } from "@solana/kit-plugin-wallet/react";
import {
  getInitializePetInstructionAsync,
  type Pet,
} from "@bichito-de-pda/client";
import type { Address } from "@solana/kit";
import type { AppClient } from "./client";
import { readPetForAuthority } from "./petService";

export function PetPanel() {
  const client = useClient<AppClient>();
  const connected = useConnectedWallet(client);
  const authority = connected?.account.address;

  const [petAddress, setPetAddress] = useState<Address | null>(null);
  const [pet, setPet] = useState<Pet | null>(null);
  const [name, setName] = useState("Pompon");
  const [loading, setLoading] = useState(true);
  const [sending, setSending] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [signature, setSignature] = useState<string | null>(null);

  useEffect(() => {
    let active = true;

    async function loadPet() {
      if (!authority) {
        setPetAddress(null);
        setPet(null);
        setLoading(false);
        return;
      }

      setLoading(true);
      setError(null);

      try {
        const result = await readPetForAuthority(authority);
        if (!active) return;
        setPetAddress(result.petAddress);
        setPet(result.pet);
      } catch (cause) {
        if (!active) return;
        setError(cause instanceof Error ? cause.message : String(cause));
      } finally {
        if (active) setLoading(false);
      }
    }

    void loadPet();

    return () => {
      active = false;
    };
  }, [authority]);

  async function createPet() {
    const signer = connected?.signer;
    const petName = name.trim();

    if (!signer || !authority) {
      setError("Conectá una wallet que permita firmar transacciones.");
      return;
    }

    const nameBytes = new TextEncoder().encode(petName).length;
    if (nameBytes < 1 || nameBytes > 32) {
      setError("El nombre debe tener entre 1 y 32 bytes UTF-8.");
      return;
    }

    setSending(true);
    setError(null);
    setSignature(null);

    try {
      const instruction = await getInitializePetInstructionAsync({
        authority: signer,
        name: petName,
      });

      const result = await client.sendTransaction([instruction]);
      setSignature(result.context.signature);

      const updated = await readPetForAuthority(authority);
      setPetAddress(updated.petAddress);
      setPet(updated.pet);
    } catch (cause) {
      setError(cause instanceof Error ? cause.message : String(cause));
    } finally {
      setSending(false);
    }
  }

  return (
    <section className="pet-panel" aria-labelledby="pet-panel-title">
      <p className="eyebrow">Tu espacio en Solana</p>

      {loading ? (
        <p className="pet-message" role="status">
          Buscando la PDA de tu mascota…
        </p>
      ) : pet ? (
        <>
          <h2 id="pet-panel-title">¡Hola, {pet.name}!</h2>
          <p className="pet-message">
            Encontramos su cuenta en Devnet. Cuidados registrados:{" "}
            <strong>{pet.careCount} de 3</strong>.
          </p>
          {petAddress && (
            <p className="pet-address">
              PDA: {petAddress.slice(0, 8)}…{petAddress.slice(-6)}
            </p>
          )}
        </>
      ) : (
        <>
          <h2 id="pet-panel-title">Todavía no tenés un bichito</h2>
          <p className="pet-message">
            Elegí un nombre y registrá su cuenta PDA en Devnet.
          </p>
          <label className="pet-label" htmlFor="pet-name">
            Nombre de tu mascota
          </label>
          <div className="pet-create-row">
            <input
              id="pet-name"
              className="pet-input"
              value={name}
              onChange={(event) => setName(event.target.value)}
              maxLength={32}
              placeholder="Por ejemplo, Pompon"
              disabled={sending}
            />
            <button
              className="button button-primary"
              type="button"
              onClick={() => void createPet()}
              disabled={sending || !connected?.signer}
            >
              {sending ? "Esperando confirmación…" : "Crear mi bichito"}
            </button>
          </div>
        </>
      )}

      {error && <p className="wallet-error" role="alert">{error}</p>}

      {signature && (
        <p className="pet-message" role="status">
          Transacción confirmada.{" "}
          <a
            href={`https://explorer.solana.com/tx/${signature}?cluster=devnet`}
            target="_blank"
            rel="noreferrer"
          >
            Ver en Solana Explorer
          </a>
        </p>
      )}
    </section>
  );
}