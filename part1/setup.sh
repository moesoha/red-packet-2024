# create a netns for SRv6 decapsulate
ip netns add fbacvpn
# and we need to forward Web IPv6 to main instance
ip netns exec fbacvpn sysctl -w net.ipv6.conf.all.forwarding=1

# connect SRv6 router and Web server
ip link add veth-vpn type veth peer name veth-main
ip link set veth-main netns fbacvpn
ip link set dev veth-vpn up
ip -n fbacvpn link set dev veth-main up
ip addr add dev veth-vpn fe80::c:1/64
ip -n fbacvpn addr add dev veth-main fe80::c:2/64
# make it possible to leak traffic to Web, eth0 is the interface for Web
ip -n fbacvpn route add dev veth-main 2402:4e00:1801:ef0c:0:9b49:8249:e7e8/128 via fe80::c:1 # Web IP
# drop direct access to port 81 from eth0
/etc/nftables.conf
nft add rule inet filter input iifname eth0 tcp dport 81 drop

# eth1 is the interface for SRv6 ingress
ip link set eth1 netns fbacvpn
ip -n fbacvpn link set dev eth1 up
# SRv6 should be enabled. HMAC setting should be 0: only validate segments with HMAC, and allow segments without HMAC
ip netns exec fbacvpn sysctl -w net.ipv6.conf.eth1.seg6_enabled=1 net.ipv6.conf.eth1.seg6_require_hmac=0
# dummy interface to associate SRv6 functions
ip -n fbacvpn link add dummy-srv6 type dummy
ip -n fbacvpn link set dev dummy-srv6 up
ip -n fbacvpn -6 route add 2402:4e00:1801:ef0c::fbac:2024/128 encap seg6local action End.DX6 nh6 :: dev dummy-srv6 table main