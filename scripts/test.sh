# A script to test quickly

killall {node} &> /dev/null
rm -rf /tmp/*.db &> /dev/null
vals=(27000 27100 27200 27300)

#rand=$(gshuf -i 1000-150000000 -n 1)
TESTDIR=${TESTDIR:="testdata/hyb_16"}
TYPE=${TYPE:="release"}

# check if $5 is set, otherwise set to rbc
protocol=${5:-rbc} 


# Run the syncer now
./target/$TYPE/node \
    --config $TESTDIR/nodes-0.json \
    --ip ip_file \
    --protocol sync \
    --input 100 \
    --syncer $1 \
    --bfile $4 \
    --byzantine false > logs/syncer.log &

for((i=0;i<16;i++)); do
./target/$TYPE/node \
    --config $TESTDIR/nodes-$i.json \
    --ip ip_file \
    --protocol $protocol \
    --input $2 \
    --syncer $1 \
    --bfile $4 \
    --byzantine $3 > logs/$i.log &
done

# Kill all nodes sudo lsof -ti:7000-7015 | xargs kill -9
# options for $5: rbc, ctrbc, ecc_rbc