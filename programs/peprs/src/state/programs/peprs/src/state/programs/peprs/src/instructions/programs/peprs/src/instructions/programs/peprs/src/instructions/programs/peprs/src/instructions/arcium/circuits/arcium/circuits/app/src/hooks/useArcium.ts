import { ArciumClient } from '@arcium-hq/arcium-client';
import { useConnection, useWallet } from '@solana/wallet-adapter-react';
import { useCallback } from 'react';

export function useArcium() {
  const { connection } = useConnection();
  const { publicKey } = useWallet();

  const client = new ArciumClient({
    connection,
    cluster: 'devnet',
  });

  const submitPrivateOrder = useCallback(async (params: {
    size: number;
    price: number;
    direction: 'long' | 'short';
    leverage: number;
    market: string;
  }) => {
    if (!publicKey) throw new Error('Wallet not connected');

    const jobId = await client.submitJob({
      circuit: 'order_match',
      inputs: {
        size: params.size,
        price: params.price,
        leverage: params.leverage,
      },
      publicInputs: {
        direction: params.direction === 'long' ? 0 : 1,
        market: params.market,
      },
    });

    const commitment = await client.generateCommitment(
      params.size,
      params.price
    );

    return { jobId, commitment };
  }, [publicKey, client]);

  const closePrivatePosition = useCallback(async (arciumJobId: string) => {
    const result = await client.resolveJob({
      jobId: arciumJobId,
      circuit: 'pnl_settle',
    });

    return {
      proof: result.proof,
      realizedPnl: result.output.realized_pnl,
    };
  }, [client]);

  const checkLiquidation = useCallback(async (params: {
    arciumJobId: string;
    currentPrice: number;
    maintenanceMarginBps: number;
  }) => {
    const result = await client.resolveJob({
      jobId: params.arciumJobId,
      circuit: 'liq_check',
      publicInputs: {
        current_price: params.currentPrice,
        maintenance_margin_bps: params.maintenanceMarginBps,
      },
    });

    return result.output.is_liquidatable as boolean;
  }, [client]);

  return {
    submitP
