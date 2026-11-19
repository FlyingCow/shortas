{$DASHBOARD_HOSTNAME} {
	reverse_proxy dashboard:80
}

{$LANDING_HOSTNAME} {
	reverse_proxy landing:80
}

{$API_HOSTNAME} {
	reverse_proxy api:8080
}
