variable "load_balancer_name" {
  type    = string
  default = "TrackBackDevChain"
}

variable "certificate_arn" {
  type    = string
  default = "arn:aws:acm:ap-southeast-2:533545012068:certificate/4fc4d08a-913c-468b-a9b2-69475b142193"
}

variable "zone_id" {
  type    = string
  default = "Z08514031O6MGON8YFSCB"
}

variable "domain" {
  type    = string
  default = "n01.trackback.dev"
}


