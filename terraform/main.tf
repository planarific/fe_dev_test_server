#provider "azurerm" {
#  version         = "=1.24.0"
#  subscription_id = "${var.subscription_id}"
#}

# Configure the Azure provider
terraform {
  required_providers {
    azurerm = {
      source  = "hashicorp/azurerm"
      version = "~> 3.0.0"
    }
  }
  required_version = ">= 0.14.9"
}

provider "azurerm" {
  features {}
}

resource "azurerm_resource_group" "main" {
  name     = "${var.prefix}-resources"
  location = "${var.location}"
}

# This creates the plan that the service use
resource "azurerm_service_plan" "main" {
  name                = "${var.prefix}-asp"
  resource_group_name = "${azurerm_resource_group.main.name}"
  location            = "${azurerm_resource_group.main.location}"
  os_type             = "Linux"
  sku_name            = "B1"

}

# This creates the service definition
resource "azurerm_linux_web_app" "main" {
  name                = "${var.prefix}-webapp"
  resource_group_name = "${azurerm_resource_group.main.name}"
  location            = "${azurerm_resource_group.main.location}"
  app_service_plan_id = "${azurerm_app_service_plan.main.id}"

  site_config {
    # app_command_line = ""
    # linux_fx_version = "DOCKER|${var.docker_image}:${var.docker_image_tag}"
    # always_on        = true
  }

  app_settings = {
    "WEBSITES_ENABLE_APP_SERVICE_STORAGE" = "false"
    "DOCKER_REGISTRY_SERVER_URL"          = "https://index.docker.io"

    # These are app specific environment variables
  }
}
