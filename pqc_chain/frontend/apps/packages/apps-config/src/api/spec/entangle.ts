// Copyright 2017-2026 @polkadot/apps-config authors & contributors
// SPDX-License-Identifier: Apache-2.0

import type { OverrideBundleDefinition } from '@polkadot/types/types';

// structs need to be in order
/* eslint-disable sort-keys */

const definitions: OverrideBundleDefinition = {
  types: [
    {
      minmax: [0, undefined],
      types: {
        HybridPublic: {
          _enum: {
            Classic: 'MultiSigner',
            MlDsa65: 'MlDsaPublicKey'
          }
        },
        HybridSignature: {
          _enum: {
            Classic: 'MultiSignature',
            MlDsa65: 'MlDsaSignature'
          }
        },
        MlDsaPublicKey: '[u8; 1952]',
        MlDsaSecretKey: '[u8; 32]',
        MlDsaSignature: 'Bytes',
        MlKemCiphertext: '[u8; 1088]',
        MlKemPublicKey: '[u8; 1184]',
        MlKemSecretKey: '[u8; 64]',
        MlKemSharedSecret: '[u8; 32]',
        SignatureScheme: {
          _enum: [
            'Classic',
            'MlDsa65'
          ]
        }
      }
    }
  ]
};

export default definitions;