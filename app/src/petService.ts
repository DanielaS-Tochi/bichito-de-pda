import {
  address,
  getAddressEncoder,
  getProgramDerivedAddress,
  getUtf8Encoder,
  type Address,
} from "@solana/kit";
import {
  BICHITO_DE_PDA_PROGRAM_ADDRESS,
  fetchMaybePet,
  type Pet,
} from "@bichito-de-pda/client";
import { client } from "./client";

export async function readPetForAuthority(authority: string): Promise<{
  petAddress: Address;
  pet: Pet | null;
}> {
  const [petAddress] = await getProgramDerivedAddress({
    programAddress: BICHITO_DE_PDA_PROGRAM_ADDRESS,
    seeds: [
      getUtf8Encoder().encode("pet"),
      getAddressEncoder().encode(address(authority)),
    ],
  });

  const account = await fetchMaybePet(client, petAddress);

  return {
    petAddress,
    pet: account.exists ? account.data : null,
  };
}
