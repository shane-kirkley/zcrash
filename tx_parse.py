import json
import sys

with open(sys.argv[1]) as f:
    j = json.load(f)

tx_hex = j['data']['tx_hex']

# parse tx_hex according to transaction format
header = tx_hex[:4*2]
tx_in_count = tx_hex[4*2:4*2 + 2]
tx_out_count = tx_hex[10:12]
time = tx_hex[12:20]
nJoinSplit = tx_hex[20:22]
joinSplitSig = tx_hex[-64*2:]
joinSplitPubKey = tx_hex[-64*2 - 32*2: -64*2]
vJoinSplit = tx_hex[-64*2 - 32*2 - 1802*2: -64*2 - 32*2]

# vJoinSplit is a joinsplit description, parse according to protocol 6.2
vpub_old = vJoinSplit[:16]
vpub_new = vJoinSplit[16:32]
anchor = vJoinSplit[32:96]
nullifiers = vJoinSplit[96:224]
commitments = vJoinSplit[224:352]
ephemeralKey = vJoinSplit[352:416]
randomSeed = vJoinSplit[416:480]
vmacs = vJoinSplit[480:608]
zkproof = vJoinSplit[608:1200]
encChiphertexts = vJoinSplit[1200:]

# zkproof is BCTV14 proof, parse according to protocol
# 264-bit pi_a and pi_a_prime
pi_a = zkproof[:66]
pi_a_prime = zkproof[66:132]
# 520-bit pi_b
pi_b = zkproof[132:262]
# 264-bit for the rest of pi
pi_b_prime = zkproof[262:328]
pi_c = zkproof[328:394]
pi_c_prime = zkproof[394:460]
pi_k = zkproof[460:526]
pi_h = zkproof[526:]

# now we calculate nu_a and nu_a_prime?
