import type { NodeSubtype, NoFallbackBehavior, WmNode } from '@seelen-ui/types';
import { enumFromUnion } from '../utils/enums.ts';

// =================================================================================
//    From here some enums as helpers like @seelen-ui/types only contains types
// =================================================================================

const NodeType = enumFromUnion<WmNode['type']>({
  Vertical: 'Vertical',
  Horizontal: 'Horizontal',
  Leaf: 'Leaf',
  Stack: 'Stack',
  Fallback: 'Fallback',
});

const NodeSubtype = enumFromUnion<NodeSubtype>({
  Temporal: 'Temporal',
  Permanent: 'Permanent',
});

const NoFallbackBehavior = enumFromUnion<NoFallbackBehavior>({
  Float: 'Float',
  Unmanaged: 'Unmanaged',
});

export { NodeSubtype, NodeType, NoFallbackBehavior };
