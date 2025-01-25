import { server } from "@/modules"
import { TokenSwap } from "@/modules/orca/token-swap"
import { ManagedPosition } from "@/modules/pool-manager/managed-position"
import { NewPosition, NewProgrammaticPosition } from "@/modules/pool-manager/new-position"
import { OrcaPool } from "@/modules/pool-manager/orca-pool"

const resource = 'pool_manager'

export default {
    get: {
        allPositions: (): Promise<ManagedPosition[]> => {
            return new Promise((ok, err) => {
                server.get(resource, 'all-positions', {})
                    .then(r => {
                        console.log('response', r)
                        ok(r.data)
                    })
                    .catch(e => err(e))
            })
        },
        orcaPools: (limit?: number): Promise<OrcaPool[]> => {
            return new Promise((ok, err) => {
                fetch('https://stats-api.mainnet.orca.so/api/whirlpools?limit=' + (limit || 50) + '&sort=volume:desc')
                    .then(response => response.json())
                    .then((data: { data: OrcaPool[], meta: any }) => {
                        ok(data.data)
                    })
            })
        },
        programmaticWalletPubkey: (): Promise<string> => {
            return new Promise((ok, err) => {
                server.get(resource, 'programmatic-wallet-pubkey', {})
                    .then(r => ok(r.data))
                    .catch(e => err(e))
            })
        },
        storedLocalWalletPubkey: (): Promise<string> => {
            return new Promise((ok, err) => {
                server.get(resource, 'stored-local-wallet-pubkey', {})
                    .then(r => ok(r.data))
                    .catch(e => err(e))
            })
        },
        openPositionInstruction: (position: NewPosition): Promise<any> => {
            return new Promise((ok, err) => {
                server.post(resource, 'open-position-instruction', position.toSnakeCase())
                    .then(r => ok(r.data))
                    .catch(e => err(e))
            })
        }
    },
    openPosition: (position: NewPosition): Promise<any> => {
        return new Promise((ok, err) => {
            server.post(resource, 'open-position', position.toSnakeCase())
                .then(r => ok(r.data))
                .catch(e => err(e))
        })
    },
    closePosition: (position: ManagedPosition): Promise<any> => {
        return new Promise((ok, err) => {
            server.put(resource, 'close-position', position.toSnakeCase())
                .then(r => ok(r.data))
                .catch(e => err(e))
        })
    },
    toggleAutoRebalance: (position: ManagedPosition): Promise<void> => {
        return new Promise((ok, err) => {
            console.log('toggling auto-rebalance', position.toSnakeCase())
            server.put(resource, 'toggle-auto-rebalance', position.toSnakeCase())
                .then(_r => ok())
                .catch(e => err(e))
        })
    },
    openProgrammaticPosition: (position: NewProgrammaticPosition): Promise<any> => {
        return new Promise((ok, err) => {
            server.post(resource, 'open-programmatic-position', position.toSnakeCase())
                .then(r => ok(r.data))
                .catch(e => err(e))
        })
    },
    connectLocalWallet: (walletKey: string): Promise<ManagedPosition[]> => {
        console.log('connecting wallet', walletKey)
        return new Promise((ok, err) => {
            server.put(resource, 'connect-local-wallet', { wallet_key: walletKey})
                .then(r => {
                    const managedPositions = r.data.map((position: any) => new ManagedPosition(position));
                    console.log('connected wallet', managedPositions)
                    ok(managedPositions)
                })
                .catch(e => err(e))
        })
    },
    disconnectLocalWallet: (): Promise<ManagedPosition[]> => {
        return new Promise((ok, err) => {
            server.put(resource, 'disconnect-local-wallet', {})
                .then(r => {
                    const removedPositions = r.data.map((position: any) => new ManagedPosition(position));
                    console.log('disconnected wallet', removedPositions)
                    ok(removedPositions)
                })
                .catch(e => err(e))
        })
    },
    swapTokens: (tokenSwap: TokenSwap): Promise<any> => {
        return new Promise((ok, err) => {
            server.post(resource, 'swap-tokens', tokenSwap.toSnakeCase())
                .then(r => ok(r.data))
                .catch(e => err(e))
        })
    }
}