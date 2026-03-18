#!/usr/bin/env bash
cd "$(dirname "$0")"

echo "# BUILDING LOCAL INFRASTRUCTURE##################################################"
echo ""
echo "# INSTALLING TOOLS---------------------------------------------------------------"


chmod u+x ./install-software.sh
./install-software.sh

echo ""
echo "# RUNNING DOCKER------------------------------------------------------------------"

docker compose -f ./docker-services.local.yml up -d 

echo ""
echo "# CREATING ENVIRONMENT------------------------------------------------------------"

chmod u+x ./install-environment.sh
./install-environment.sh

echo ""
echo "# SEEDING SOME DATA---------------------------------------------------------------"

chmod u+x ./seed-test-data.sh
./seed-test-data.sh

echo "# DONE BUILDING LOCAL INFRASTRUCTURE##############################################"