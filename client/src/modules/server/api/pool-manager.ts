import { server } from "@/modules"
import { ManagedPosition } from "@/modules/pool-manager/managed-position"
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
        }
    }
}